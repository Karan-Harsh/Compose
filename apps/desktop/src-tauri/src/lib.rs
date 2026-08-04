mod commands;
mod error;
mod services;
mod state;

use state::app_state::AppState;

pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_info,
            commands::app::health_check,
            commands::window::show_palette
        ])
        .run(tauri::generate_context!())
        .expect("failed to run TypeFlow desktop application");
}
