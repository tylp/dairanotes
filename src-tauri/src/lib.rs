use std::sync::Mutex;

use note::NoteManager;
use tauri::{Builder, Manager};

mod api;
mod commands;
mod note;

#[derive(Debug, Default)]
struct AppState {
    notes_manager: NoteManager,
}

impl AppState {
    pub fn notes_manager(&self) -> &NoteManager {
        &self.notes_manager
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::notes_index,
            commands::notes_show,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
