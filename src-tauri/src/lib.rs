use http::HttpClientImpl;
use tauri::{Builder, Manager};
use tauri_plugin_store::StoreExt;

mod commands;
mod http;
mod note;
mod service;
mod user;

use note::note_service::LocalNoteService;
use note::note_service::RemoteNoteService;
use note::NoteManager;
use user::user_service::LocalUserService;
use user::user_service::RemoteUserService;
use user::UserManager;

struct AppState {
    notes_manager: NoteManager,
    users_manager: UserManager,
}

impl AppState {
    pub fn notes_manager(&self) -> &NoteManager {
        &self.notes_manager
    }

    pub fn users_manager(&self) -> &UserManager {
        &self.users_manager
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

            let user_store = app.handle().store_builder("users.bin").build();
            let local_user_service = LocalUserService::new(user_store);
            let _remote_user_service =
                RemoteUserService::new("/users".to_string(), HttpClientImpl::default());

            let notes_manager = NoteManager::new(Box::new(local_note_service));
            let users_manager = UserManager::new(Box::new(local_user_service));

            let state = AppState {
                notes_manager,
                users_manager,
            };

            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::notes_commands::notes_index,
            commands::notes_commands::notes_show,
            commands::users_commands::users_index,
            commands::users_commands::users_show
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
