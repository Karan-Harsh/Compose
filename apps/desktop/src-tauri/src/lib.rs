mod commands;
mod error;
mod platform;
mod services;
mod state;

use state::app_state::AppState;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Builder as GlobalShortcutBuilder, ShortcutState};

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_state = AppState::initialize(app.handle())
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            app.manage(app_state);

            app.handle().plugin(
                GlobalShortcutBuilder::new()
                    .with_handler(|app, _shortcut, event| {
                        if event.state != ShortcutState::Pressed {
                            return;
                        }

                        if let Some(state) = app.try_state::<AppState>() {
                            // Capture while the previous app still has focus.
                            let _ = state
                                .accessibility_service
                                .capture_and_remember(&state.clipboard_service);
                            let _ = state.window_service.show_main_window(app);
                        }
                    })
                    .build(),
            )?;

            let state = app.state::<AppState>();
            state
                .window_service
                .configure_palette_behavior(app.handle())
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;
            state
                .hotkey_service
                .bootstrap(app.handle(), &state.settings_service)
                .map_err(|error| -> Box<dyn std::error::Error> { Box::new(error) })?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_info,
            commands::app::health_check,
            commands::window::show_palette,
            commands::window::hide_palette,
            commands::settings::load_settings,
            commands::settings::save_settings,
            commands::clipboard::get_clipboard_text,
            commands::clipboard::set_clipboard_text,
            commands::hotkey::get_hotkey_status,
            commands::hotkey::set_global_hotkey,
            commands::accessibility::get_selected_text,
            commands::accessibility::get_last_selection,
            commands::ai::list_ai_providers,
            commands::ai::get_active_ai_provider,
            commands::ai::set_active_ai_provider,
            commands::ai::list_commands,
            commands::ai::complete_ai
        ])
        .run(tauri::generate_context!())
        .expect("failed to run TypeFlow desktop application");
}
