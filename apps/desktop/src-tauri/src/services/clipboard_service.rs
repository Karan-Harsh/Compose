use arboard::Clipboard;

use crate::error::app_error::{AppError, AppResult};

#[derive(Debug, Default)]
pub struct ClipboardService;

impl ClipboardService {
    pub fn get_text(&self) -> AppResult<String> {
        let mut clipboard =
            Clipboard::new().map_err(|error| AppError::Clipboard(error.to_string()))?;

        clipboard
            .get_text()
            .map_err(|error| AppError::Clipboard(error.to_string()))
    }

    pub fn set_text(&self, text: &str) -> AppResult<()> {
        let mut clipboard =
            Clipboard::new().map_err(|error| AppError::Clipboard(error.to_string()))?;

        clipboard
            .set_text(text)
            .map_err(|error| AppError::Clipboard(error.to_string()))
    }
}
