use serde::Serialize;
use tauri::State;

use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedTextResponse {
    pub text: String,
    pub method: String,
    pub empty: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionCaptureResponse {
    pub text: String,
    pub method: String,
    pub empty: bool,
    pub captured_at_ms: u64,
}

#[tauri::command]
pub fn get_selected_text(state: State<'_, AppState>) -> Result<SelectedTextResponse, String> {
    state
        .accessibility_service
        .get_selected_text(&state.clipboard_service)
        .map(|selection| SelectedTextResponse {
            text: selection.text,
            method: selection.method,
            empty: selection.empty,
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_last_selection(
    state: State<'_, AppState>,
) -> Result<Option<SelectionCaptureResponse>, String> {
    state
        .accessibility_service
        .last_capture()
        .map(|capture| {
            capture.map(|value| SelectionCaptureResponse {
                text: value.text,
                method: value.method,
                empty: value.empty,
                captured_at_ms: value.captured_at_ms,
            })
        })
        .map_err(|error| error.to_string())
}
