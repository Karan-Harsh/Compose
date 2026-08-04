use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("main window is not available")]
    WindowUnavailable,
    #[error("native window operation failed: {0}")]
    Tauri(#[from] tauri::Error),
}

pub type AppResult<T> = Result<T, AppError>;
