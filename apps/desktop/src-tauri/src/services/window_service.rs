use tauri::{AppHandle, Manager, Runtime, WindowEvent};

use crate::error::app_error::{AppError, AppResult};

#[derive(Debug, Default)]
pub struct WindowService;

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

        let hide_target = window.clone();
        window.on_window_event(move |event| {
            if let WindowEvent::Focused(false) = event {
                let _ = hide_target.hide();
            }
        });

        Ok(())
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
}
