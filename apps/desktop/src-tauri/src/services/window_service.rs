use tauri::{AppHandle, Manager, Runtime};

use crate::error::app_error::{AppError, AppResult};

#[derive(Debug, Default)]
pub struct WindowService;

impl WindowService {
    pub fn show_main_window<R: Runtime>(&self, app_handle: &AppHandle<R>) -> AppResult<()> {
        let window = app_handle
            .get_webview_window("main")
            .ok_or(AppError::WindowUnavailable)?;

        window.show()?;
        window.unminimize()?;
        window.set_focus()?;

        Ok(())
    }
}
