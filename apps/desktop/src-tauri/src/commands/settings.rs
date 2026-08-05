use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime, State};

use crate::services::settings_service::AppSettings;
use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub launch_at_login: bool,
    pub global_hotkey: String,
    pub theme: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveSettingsRequest {
    pub launch_at_login: bool,
    pub global_hotkey: String,
    pub theme: String,
}

impl From<AppSettings> for SettingsResponse {
    fn from(value: AppSettings) -> Self {
        Self {
            launch_at_login: value.launch_at_login,
            global_hotkey: value.global_hotkey,
            theme: value.theme,
        }
    }
}

#[tauri::command]
pub fn load_settings(state: State<'_, AppState>) -> Result<SettingsResponse, String> {
    state
        .settings_service
        .load()
        .map(SettingsResponse::from)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn save_settings<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, AppState>,
    request: SaveSettingsRequest,
) -> Result<SettingsResponse, String> {
    let previous = state
        .settings_service
        .load()
        .map_err(|error| error.to_string())?;

    let settings = AppSettings {
        launch_at_login: request.launch_at_login,
        global_hotkey: request.global_hotkey,
        theme: request.theme,
    };

    let saved = state
        .settings_service
        .save(&settings)
        .map_err(|error| error.to_string())?;

    if previous.global_hotkey != saved.global_hotkey {
        state
            .hotkey_service
            .register(&app, &saved.global_hotkey)
            .map_err(|error| error.to_string())?;
    }

    Ok(SettingsResponse::from(saved))
}
