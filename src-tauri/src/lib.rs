use http::HttpClientImpl;
use note::NoteManager;
use tauri::{Builder, Manager};
use tauri_plugin_store::StoreExt;

mod commands;
mod http;
mod note;
mod service;

use note::note_service::LocalNoteService;
use note::note_service::RemoteNoteService;

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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let notes_store = app.handle().store_builder("notes.bin").build();

            let local_note_service = LocalNoteService::new(notes_store);
            let _remote_note_service =
                RemoteNoteService::new("/notes".to_string(), HttpClientImpl::default());

            let note_manager = NoteManager::new(Box::new(local_note_service));

            let state = AppState {
                notes_manager: note_manager,
            };

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::notes_index,
            commands::notes_show,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
