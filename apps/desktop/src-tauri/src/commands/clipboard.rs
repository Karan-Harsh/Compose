use serde::{Deserialize, Serialize};
use tauri::State;

use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardTextResponse {
    pub text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetClipboardTextRequest {
    pub text: String,
}

#[tauri::command]
pub fn get_clipboard_text(state: State<'_, AppState>) -> Result<ClipboardTextResponse, String> {
    state
        .clipboard_service
        .get_text()
        .map(|text| ClipboardTextResponse { text })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn set_clipboard_text(
    state: State<'_, AppState>,
    request: SetClipboardTextRequest,
) -> Result<(), String> {
    state
        .clipboard_service
        .set_text(&request.text)
        .map_err(|error| error.to_string())
}
