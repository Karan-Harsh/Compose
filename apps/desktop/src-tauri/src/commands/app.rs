use serde::Serialize;
use tauri::State;

use crate::state::app_state::AppState;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppInfoResponse {
    pub name: String,
    pub version: String,
    pub os: String,
    pub arch: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    pub status: String,
}

#[tauri::command]
pub fn get_app_info(state: State<'_, AppState>) -> Result<AppInfoResponse, String> {
    state
        .app_info_service
        .get_app_info()
        .map(|info| AppInfoResponse {
            name: info.name,
            version: info.version,
            os: info.os,
            arch: info.arch,
        })
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn health_check(state: State<'_, AppState>) -> Result<HealthCheckResponse, String> {
    state
        .app_info_service
        .health_check()
        .map(|health| HealthCheckResponse {
            status: health.status,
        })
        .map_err(|error| error.to_string())
}
