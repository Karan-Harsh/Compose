use std::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime, WindowEvent};

use crate::error::app_error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaletteOpenedPayload {
    pub resumed: bool,
}

#[derive(Debug, Default)]
pub struct WindowService {
    /// When true, blur must not auto-hide (AI running or result under review).
    session_active: AtomicBool,
}

impl WindowService {
    pub fn configure_palette_behavior<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
    ) -> AppResult<()> {
        let window = app_handle
            .get_webview_window("main")
            .ok_or(AppError::WindowUnavailable)?;

        window.set_always_on_top(true)?;
        window.set_skip_taskbar(true)?;
        let _ = window.set_decorations(false);

        let hide_target = window.clone();
        let app_for_flag = app_handle.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::Focused(false) = event {
                let Some(state) = app_for_flag.try_state::<crate::state::app_state::AppState>()
                else {
                    return;
                };
                if state.window_service.is_session_active() {
                    eprintln!("TypeFlow: blur ignored (session active)");
                    return;
                }
                let _ = hide_target.hide();
            }
        });

        Ok(())
    }

    pub fn set_session_active(&self, active: bool) {
        self.session_active.store(active, Ordering::SeqCst);
        eprintln!("TypeFlow: palette session_active={active}");
    }

    pub fn is_session_active(&self) -> bool {
        self.session_active.load(Ordering::SeqCst)
    }

    pub fn is_main_visible<R: Runtime>(&self, app_handle: &AppHandle<R>) -> bool {
        app_handle
            .get_webview_window("main")
            .and_then(|window| window.is_visible().ok())
            .unwrap_or(false)
    }

    pub fn show_main_window<R: Runtime>(&self, app_handle: &AppHandle<R>) -> AppResult<()> {
        let window = app_handle
            .get_webview_window("main")
            .ok_or(AppError::WindowUnavailable)?;

        window.set_always_on_top(true)?;
        window.show()?;
        window.unminimize()?;
        window.set_focus()?;

        Ok(())
    }

    pub fn hide_main_window<R: Runtime>(&self, app_handle: &AppHandle<R>) -> AppResult<()> {
        let window = app_handle
            .get_webview_window("main")
            .ok_or(AppError::WindowUnavailable)?;

        window.hide()?;
        Ok(())
    }

    pub fn open_palette_from_hotkey<R: Runtime>(
        &self,
        app_handle: &AppHandle<R>,
        accessibility: &crate::services::accessibility_service::AccessibilityService,
        clipboard: &crate::services::clipboard_service::ClipboardService,
    ) -> AppResult<bool> {
        let visible = self.is_main_visible(app_handle);
        let resumed = visible || self.is_session_active();

        if resumed {
            eprintln!("TypeFlow: hotkey resume (visible={visible})");
            self.show_main_window(app_handle)?;
        } else {
            eprintln!("TypeFlow: hotkey new session — capturing selection");
            let _ = accessibility.capture_and_remember(clipboard);
            self.show_main_window(app_handle)?;
        }

        let _ = app_handle.emit(
            "palette-opened",
            PaletteOpenedPayload { resumed },
        );

        Ok(resumed)
    }
}
