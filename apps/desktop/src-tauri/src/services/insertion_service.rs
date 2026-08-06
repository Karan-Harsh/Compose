use std::thread;
use std::time::Duration;

use serde::Serialize;

use crate::error::app_error::{AppError, AppResult};
use crate::platform::{focus, input};
use crate::services::accessibility_service::AccessibilityService;
use crate::services::clipboard_service::ClipboardService;

const PRE_PASTE_DELAY: Duration = Duration::from_millis(60);
const POST_PASTE_DELAY: Duration = Duration::from_millis(140);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InsertionResult {
    pub method: String,
    pub restored_clipboard: bool,
}

#[derive(Debug, Default)]
pub struct InsertionService;

impl InsertionService {
    pub fn insert_via_clipboard_paste(
        &self,
        clipboard: &ClipboardService,
        accessibility: &AccessibilityService,
        text: &str,
    ) -> AppResult<InsertionResult> {
        if text.is_empty() {
            return Err(AppError::Insertion(
                "cannot insert empty text".to_string(),
            ));
        }

        if let Some(source_app) = accessibility.source_app()? {
            eprintln!(
                "TypeFlow: activating source app before paste name={} pid={}",
                source_app.name, source_app.pid
            );
            focus::activate_app(&source_app)?;
        } else {
            eprintln!(
                "TypeFlow: no remembered source app; pasting into whatever is frontmost"
            );
        }

        let previous = clipboard.get_text().unwrap_or_default();
        clipboard.set_text(text)?;
        eprintln!(
            "TypeFlow: clipboard prepared for paste chars={}",
            text.chars().count()
        );
        thread::sleep(PRE_PASTE_DELAY);

        input::simulate_paste().map_err(|error| {
            eprintln!("TypeFlow: simulate_paste failed: {error}");
            error
        })?;
        eprintln!("TypeFlow: simulate_paste sent (Cmd/Ctrl+V)");
        thread::sleep(POST_PASTE_DELAY);

        let restored_clipboard = clipboard.set_text(&previous).is_ok();

        Ok(InsertionResult {
            method: "clipboard_paste".to_string(),
            restored_clipboard,
        })
    }
}
