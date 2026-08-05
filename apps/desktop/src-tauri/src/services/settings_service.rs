use std::path::Path;
use std::sync::Mutex;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::error::app_error::{AppError, AppResult};

const SETTINGS_KEY: &str = "app_settings";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub launch_at_login: bool,
    pub global_hotkey: String,
    pub theme: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            launch_at_login: false,
            global_hotkey: "CommandOrControl+Shift+Space".to_string(),
            theme: "system".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct SettingsService {
    conn: Mutex<Connection>,
}

impl SettingsService {
    pub fn open(app_data_dir: &Path) -> AppResult<Self> {
        std::fs::create_dir_all(app_data_dir)?;

        let db_path = app_data_dir.join("typeflow.db");
        let conn = Connection::open(&db_path)?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS kv (
                key TEXT PRIMARY KEY NOT NULL,
                value TEXT NOT NULL
            );",
        )?;

        let service = Self {
            conn: Mutex::new(conn),
        };
        service.ensure_defaults()?;
        Ok(service)
    }

    pub fn load(&self) -> AppResult<AppSettings> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AppError::InvalidSettings("settings lock poisoned".to_string()))?;

        let value: Option<String> = conn
            .query_row(
                "SELECT value FROM kv WHERE key = ?1",
                [SETTINGS_KEY],
                |row| row.get(0),
            )
            .optional_row()?;

        match value {
            Some(raw) => {
                let settings: AppSettings = serde_json::from_str(&raw)?;
                validate_settings(&settings)?;
                Ok(settings)
            }
            None => Ok(AppSettings::default()),
        }
    }

    pub fn save(&self, settings: &AppSettings) -> AppResult<AppSettings> {
        validate_settings(settings)?;

        let raw = serde_json::to_string(settings)?;
        let conn = self
            .conn
            .lock()
            .map_err(|_| AppError::InvalidSettings("settings lock poisoned".to_string()))?;

        conn.execute(
            "INSERT INTO kv (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            (SETTINGS_KEY, raw),
        )?;

        Ok(settings.clone())
    }

    fn ensure_defaults(&self) -> AppResult<()> {
        let existing = {
            let conn = self
                .conn
                .lock()
                .map_err(|_| AppError::InvalidSettings("settings lock poisoned".to_string()))?;

            conn.query_row(
                "SELECT value FROM kv WHERE key = ?1",
                [SETTINGS_KEY],
                |row| row.get::<_, String>(0),
            )
            .optional_row()?
        };

        if existing.is_none() {
            self.save(&AppSettings::default())?;
        }

        Ok(())
    }
}

fn validate_settings(settings: &AppSettings) -> AppResult<()> {
    if settings.global_hotkey.trim().is_empty() {
        return Err(AppError::InvalidSettings(
            "global hotkey must not be empty".to_string(),
        ));
    }

    match settings.theme.as_str() {
        "system" | "light" | "dark" => Ok(()),
        other => Err(AppError::InvalidSettings(format!(
            "unsupported theme '{other}'"
        ))),
    }
}

trait OptionalRow<T> {
    fn optional_row(self) -> AppResult<Option<T>>;
}

impl<T> OptionalRow<T> for Result<T, rusqlite::Error> {
    fn optional_row(self) -> AppResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(error) => Err(AppError::Database(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        std::env::temp_dir().join(format!("typeflow-settings-{nanos}"))
    }

    #[test]
    fn loads_defaults_and_persists_updates() {
        let dir = temp_dir();
        let service = SettingsService::open(&dir).expect("open db");

        let loaded = service.load().expect("load");
        assert_eq!(loaded, AppSettings::default());

        let mut updated = loaded;
        updated.theme = "dark".to_string();
        updated.launch_at_login = true;
        updated.global_hotkey = "CommandOrControl+Shift+A".to_string();

        service.save(&updated).expect("save");
        let reloaded = service.load().expect("reload");
        assert_eq!(reloaded, updated);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn rejects_invalid_theme() {
        let dir = temp_dir();
        let service = SettingsService::open(&dir).expect("open db");

        let invalid = AppSettings {
            theme: "neon".to_string(),
            ..AppSettings::default()
        };

        let error = service.save(&invalid).expect_err("should fail");
        assert!(matches!(error, AppError::InvalidSettings(_)));

        let _ = std::fs::remove_dir_all(dir);
    }
}
