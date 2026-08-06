use enigo::{Direction, Enigo, Key, Keyboard, Settings};

use crate::error::app_error::{AppError, AppResult};

pub fn simulate_copy() -> AppResult<()> {
    simulate_modified_key('c')
}

pub fn simulate_paste() -> AppResult<()> {
    simulate_modified_key('v')
}

fn simulate_modified_key(character: char) -> AppResult<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|error| AppError::Accessibility(error.to_string()))?;

    let modifier = command_modifier();

    enigo
        .key(modifier, Direction::Press)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;
    enigo
        .key(Key::Unicode(character), Direction::Click)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;
    enigo
        .key(modifier, Direction::Release)
        .map_err(|error| AppError::Accessibility(error.to_string()))?;

    Ok(())
}

fn command_modifier() -> Key {
    #[cfg(target_os = "macos")]
    {
        Key::Meta
    }

    #[cfg(not(target_os = "macos"))]
    {
        Key::Control
    }
}
