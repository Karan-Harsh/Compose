use serde::Deserialize;
use tauri::{AppHandle, Runtime, State};

use crate::state::app_state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPaletteSessionRequest {
    pub active: bool,
}

#[tauri::command]
pub fn show_palette<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state
        .window_service
        .show_main_window(&app_handle)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn hide_palette<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.window_service.set_session_active(false);
    state
        .window_service
        .hide_main_window(&app_handle)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_palette_session<R: Runtime>(
    _app_handle: AppHandle<R>,
    state: State<'_, AppState>,
    request: SetPaletteSessionRequest,
) -> Result<(), String> {
    state.window_service.set_session_active(request.active);
    Ok(())
}
