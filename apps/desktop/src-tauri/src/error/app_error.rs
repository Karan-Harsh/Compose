use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("main window is not available")]
    WindowUnavailable,
    #[error("native window operation failed: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("failed to resolve application data directory")]
    AppDataDirUnavailable,
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("settings serialization error: {0}")]
    SettingsSerde(#[from] serde_json::Error),
    #[error("invalid settings: {0}")]
    InvalidSettings(String),
    #[error("clipboard error: {0}")]
    Clipboard(String),
    #[error("hotkey error: {0}")]
    Hotkey(String),
    #[error("accessibility error: {0}")]
    Accessibility(String),
    #[error("ai error: {0}")]
    Ai(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type AppResult<T> = Result<T, AppError>;
