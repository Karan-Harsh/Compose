use std::sync::Mutex;

use serde::Serialize;
use tauri::{AppHandle, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::error::app_error::{AppError, AppResult};
use crate::services::settings_service::SettingsService;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HotkeyStatus {
    pub shortcut: String,
    pub registered: bool,
    pub activation: String,
}

#[derive(Debug, Default)]
pub struct HotkeyService {
    registered_shortcut: Mutex<Option<String>>,
}

impl HotkeyService {
    pub fn status(&self, settings: &SettingsService) -> AppResult<HotkeyStatus> {
        let settings = settings.load()?;
        let registered = self
            .registered_shortcut
            .lock()
            .map_err(|_| AppError::Hotkey("hotkey lock poisoned".to_string()))?
            .clone();

        let is_active = registered
            .as_ref()
            .is_some_and(|value| value == &settings.global_hotkey);

        Ok(HotkeyStatus {
            shortcut: settings.global_hotkey,
            registered: is_active,
            activation: if is_active {
                "active".to_string()
            } else {
                "inactive".to_string()
            },
        })
    }

    pub fn bootstrap<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        settings: &SettingsService,
    ) -> AppResult<HotkeyStatus> {
        let current = settings.load()?;
        self.register(app, &current.global_hotkey)?;
        self.status(settings)
    }

    pub fn set_shortcut<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        settings: &SettingsService,
        shortcut: String,
    ) -> AppResult<HotkeyStatus> {
        let trimmed = shortcut.trim().to_string();
        if trimmed.is_empty() {
            return Err(AppError::Hotkey(
                "global hotkey must not be empty".to_string(),
            ));
        }

        validate_shortcut(&trimmed)?;

        let mut current = settings.load()?;
        current.global_hotkey = trimmed.clone();
        settings.save(&current)?;

        self.register(app, &trimmed)?;
        self.status(settings)
    }

    pub fn register<R: Runtime>(&self, app: &AppHandle<R>, shortcut: &str) -> AppResult<()> {
        validate_shortcut(shortcut)?;

        let mut registered = self
            .registered_shortcut
            .lock()
            .map_err(|_| AppError::Hotkey("hotkey lock poisoned".to_string()))?;

        if let Some(existing) = registered.as_ref() {
            if existing == shortcut {
                return Ok(());
            }

            app.global_shortcut()
                .unregister(existing.as_str())
                .map_err(|error| AppError::Hotkey(error.to_string()))?;
            *registered = None;
        }

        app.global_shortcut()
            .register(shortcut)
            .map_err(|error| AppError::Hotkey(error.to_string()))?;

        *registered = Some(shortcut.to_string());
        Ok(())
    }
}

fn validate_shortcut(shortcut: &str) -> AppResult<()> {
    shortcut
        .parse::<Shortcut>()
        .map(|_| ())
        .map_err(|error| AppError::Hotkey(format!("invalid hotkey '{shortcut}': {error}")))
}
