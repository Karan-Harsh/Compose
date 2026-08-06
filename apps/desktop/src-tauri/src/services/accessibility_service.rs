use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::error::app_error::{AppError, AppResult};
use crate::platform::focus::{self, FrontmostApp};
use crate::platform::input;
use crate::services::clipboard_service::ClipboardService;

const CAPTURE_TIMEOUT: Duration = Duration::from_millis(350);
const CAPTURE_POLL: Duration = Duration::from_millis(25);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SelectedText {
    pub text: String,
    pub method: String,
    pub empty: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SelectionCapture {
    pub text: String,
    pub method: String,
    pub empty: bool,
    pub captured_at_ms: u64,
}

#[derive(Debug, Default)]
pub struct AccessibilityService {
    last_capture: Mutex<Option<SelectionCapture>>,
    source_app: Mutex<Option<FrontmostApp>>,
}

impl AccessibilityService {
    pub fn get_selected_text(
        &self,
        clipboard: &ClipboardService,
    ) -> AppResult<SelectedText> {
        let capture = self.capture_with_clipboard_fallback(clipboard)?;
        self.remember(capture.clone())?;
        Ok(SelectedText {
            text: capture.text,
            method: capture.method,
            empty: capture.empty,
        })
    }

    pub fn last_capture(&self) -> AppResult<Option<SelectionCapture>> {
        self.last_capture
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| AppError::Accessibility("selection lock poisoned".to_string()))
    }

    pub fn source_app(&self) -> AppResult<Option<FrontmostApp>> {
        self.source_app
            .lock()
            .map(|guard| guard.clone())
            .map_err(|_| AppError::Accessibility("source app lock poisoned".to_string()))
    }

    pub fn capture_and_remember(
        &self,
        clipboard: &ClipboardService,
    ) -> AppResult<SelectionCapture> {
        self.remember_source_app()?;
        let capture = self.capture_with_clipboard_fallback(clipboard)?;
        self.remember(capture.clone())?;
        Ok(capture)
    }

    fn remember(&self, capture: SelectionCapture) -> AppResult<()> {
        *self
            .last_capture
            .lock()
            .map_err(|_| AppError::Accessibility("selection lock poisoned".to_string()))? =
            Some(capture);
        Ok(())
    }

    fn remember_source_app(&self) -> AppResult<()> {
        let app = focus::get_frontmost_app()?;
        if let Some(ref app) = app {
            eprintln!(
                "TypeFlow: remembered source app name={} pid={}",
                app.name, app.pid
            );
        } else {
            eprintln!("TypeFlow: no frontmost app available to remember");
        }

        *self
            .source_app
            .lock()
            .map_err(|_| AppError::Accessibility("source app lock poisoned".to_string()))? = app;
        Ok(())
    }

    fn capture_with_clipboard_fallback(
        &self,
        clipboard: &ClipboardService,
    ) -> AppResult<SelectionCapture> {
        let previous = clipboard.get_text().unwrap_or_default();
        let sentinel = format!(
            "__typeflow_sentinel_{}__",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis())
                .unwrap_or(0)
        );

        clipboard.set_text(&sentinel)?;
        input::simulate_copy()?;

        let selected = wait_for_clipboard_change(clipboard, &sentinel)?;
        let _ = clipboard.set_text(&previous);

        let empty = selected.is_empty() || selected == sentinel;
        let text = if empty {
            String::new()
        } else {
            selected
        };

        Ok(SelectionCapture {
            text,
            method: "clipboard_fallback".to_string(),
            empty,
            captured_at_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_millis() as u64)
                .unwrap_or(0),
        })
    }
}

fn wait_for_clipboard_change(
    clipboard: &ClipboardService,
    sentinel: &str,
) -> AppResult<String> {
    let started = Instant::now();

    loop {
        match clipboard.get_text() {
            Ok(text) if text != sentinel => return Ok(text),
            Ok(text) => {
                if started.elapsed() >= CAPTURE_TIMEOUT {
                    return Ok(text);
                }
            }
            Err(error) => {
                if started.elapsed() >= CAPTURE_TIMEOUT {
                    return Err(error);
                }
            }
        }

        thread::sleep(CAPTURE_POLL);
    }
}
