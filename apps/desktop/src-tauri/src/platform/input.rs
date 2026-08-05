use enigo::{Direction, Enigo, Key, Keyboard, Settings};

use crate::error::app_error::{AppError, AppResult};

pub fn simulate_copy() -> AppResult<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|error| AppError::Accessibility(error.to_string()))?;

    let modifier = copy_modifier();

    enigo
        .key(modifier, Direction::Press)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;
    enigo
        .key(Key::Unicode('c'), Direction::Click)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;
    enigo
        .key(modifier, Direction::Release)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;

    Ok(())
}

fn copy_modifier() -> Key {
    #[cfg(target_os = "macos")]
    {
        Key::Meta
    }

    #[cfg(not(target_os = "macos"))]
    {
        Key::Control
    }
}
