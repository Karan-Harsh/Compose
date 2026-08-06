use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Runtime, State};

use crate::state::app_state::AppState;

const FOCUS_RETURN_DELAY: Duration = Duration::from_millis(180);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextResponse {
    pub method: String,
    pub restored_clipboard: bool,
}

#[tauri::command]
pub fn insert_text<R: Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, AppState>,
    request: InsertTextRequest,
) -> Result<InsertTextResponse, String> {
    state
        .window_service
        .hide_main_window(&app_handle)
        .map_err(|error| error.to_string())?;

    // Give the previously focused app time to become active again.
    thread::sleep(FOCUS_RETURN_DELAY);

    state
        .insertion_service
        .insert_via_clipboard_paste(&state.clipboard_service, &request.text)
        .map(|result| InsertTextResponse {
            method: result.method,
            restored_clipboard: result.restored_clipboard,
        })
        .map_err(|error| error.to_string())
}
