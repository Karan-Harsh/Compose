use tauri::{AppHandle, Runtime, State};

use crate::state::app_state::AppState;

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
