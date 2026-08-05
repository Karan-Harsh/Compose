use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, State};

use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyStatusResponse {
    pub shortcut: String,
    pub registered: bool,
    pub activation: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetGlobalHotkeyRequest {
    pub shortcut: String,
}

#[tauri::command]
pub fn get_hotkey_status(state: State<'_, AppState>) -> Result<HotkeyStatusResponse, String> {
    state
        .hotkey_service
        .status(&state.settings_service)
        .map(|status| HotkeyStatusResponse {
            shortcut: status.shortcut,
            registered: status.registered,
            activation: status.activation,
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_global_hotkey<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    request: SetGlobalHotkeyRequest,
) -> Result<HotkeyStatusResponse, String> {
    state
        .hotkey_service
        .set_shortcut(&app, &state.settings_service, request.shortcut)
        .map(|status| HotkeyStatusResponse {
            shortcut: status.shortcut,
            registered: status.registered,
            activation: status.activation,
        })
        .map_err(|error| error.to_string())
}
