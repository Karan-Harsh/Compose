use crate::error::app_error::{AppError, AppResult};
use crate::services::accessibility_service::AccessibilityService;
use crate::services::ai::AiService;
use crate::services::app_info_service::AppInfoService;
use crate::services::clipboard_service::ClipboardService;
use crate::services::hotkey_service::HotkeyService;
use crate::services::insertion_service::InsertionService;
use crate::services::settings_service::SettingsService;
use crate::services::window_service::WindowService;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug)]
pub struct AppState {
    pub app_info_service: AppInfoService,
    pub window_service: WindowService,
    pub settings_service: SettingsService,
    pub clipboard_service: ClipboardService,
    pub hotkey_service: HotkeyService,
    pub accessibility_service: AccessibilityService,
    pub insertion_service: InsertionService,
    pub ai_service: AiService,
}

impl AppState {
    pub fn initialize<R: Runtime>(app: &AppHandle<R>) -> AppResult<Self> {
        let app_data_dir = app
            .path()
            .app_data_dir()
            .map_err(|_| AppError::AppDataDirUnavailable)?;

        Ok(Self {
            app_info_service: AppInfoService,
            window_service: WindowService,
            settings_service: SettingsService::open(&app_data_dir)?,
            clipboard_service: ClipboardService,
            hotkey_service: HotkeyService::default(),
            accessibility_service: AccessibilityService::default(),
            insertion_service: InsertionService,
            ai_service: AiService::new(),
        })
    }
}
