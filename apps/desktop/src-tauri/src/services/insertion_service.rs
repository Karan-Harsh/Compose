use std::thread;
use std::time::Duration;

use serde::Serialize;

use crate::error::app_error::{AppError, AppResult};
use crate::platform::input;
use crate::services::clipboard_service::ClipboardService;

const PRE_PASTE_DELAY: Duration = Duration::from_millis(40);
const POST_PASTE_DELAY: Duration = Duration::from_millis(120);

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InsertionResult {
    pub method: String,
    pub restored_clipboard: bool,
}

#[derive(Debug, Default)]
pub struct InsertionService;

impl InsertionService {
    pub fn insert_via_clipboard_paste(
        &self,
        clipboard: &ClipboardService,
        text: &str,
    ) -> AppResult<InsertionResult> {
        if text.is_empty() {
            return Err(AppError::Insertion(
                "cannot insert empty text".to_string(),
            ));
        }

        let previous = clipboard.get_text().unwrap_or_default();
        clipboard.set_text(text)?;
        thread::sleep(PRE_PASTE_DELAY);
        input::simulate_paste()?;
        thread::sleep(POST_PASTE_DELAY);

        let restored_clipboard = clipboard.set_text(&previous).is_ok();

        Ok(InsertionResult {
            method: "clipboard_paste".to_string(),
            restored_clipboard,
        })
    }
}
