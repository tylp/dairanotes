use std::sync::Arc;

use async_trait::async_trait;
use http::HttpClientImpl;
use services::{
    note_service::{LocalNoteService, NoteService, RemoteNoteService},
    user_service::{LocalUserService, RemoteUserService, UserService},
};
use tauri::{App, Builder, Manager};
use tauri_plugin_store::StoreExt;

mod commands;
mod http;
mod note;
mod services;
mod user;

#[async_trait]
trait AppService: Send + Sync {
    fn note_service(&self) -> &NoteService;
    fn user_service(&self) -> &UserService;
}

struct LocalService {
    note_service: LocalNoteService,
    user_service: LocalUserService,
}

impl LocalService {
    fn new(app: &mut App) -> Self {
        let notes_store = app.handle().store_builder("notes.bin").build();
        let note_service = LocalNoteService::new(notes_store);

        let user_store = app.handle().store_builder("users.bin").build();
        let user_service = LocalUserService::new(user_store);

        Self {
            note_service,
            user_service,
        }
    }
}

#[async_trait]
impl AppService for LocalService {
    fn note_service(&self) -> &NoteService {
        &self.note_service
    }

    fn user_service(&self) -> &UserService {
        &self.user_service
    }
}

struct RemoteService {
    note_service: RemoteNoteService,
    user_service: RemoteUserService,
}

impl RemoteService {
    fn new() -> Self {
        let remote_note_service =
            RemoteNoteService::new("/notes".to_string(), HttpClientImpl::default());

        let remote_user_service =
            RemoteUserService::new("/users".to_string(), HttpClientImpl::default());

        Self {
            note_service: remote_note_service,
            user_service: remote_user_service,
        }
    }
}

#[async_trait]
impl AppService for RemoteService {
    fn note_service(&self) -> &NoteService {
        &self.note_service
    }

    fn user_service(&self) -> &UserService {
        &self.user_service
    }
}

enum NetworkMode {
    Local,
    Remote,
}

trait NetworkListener {
    fn notify(&mut self, mode: NetworkMode);
}

struct AppState {
    current_service: Arc<dyn AppService>,
    local_service: Arc<LocalService>,
    remote_service: Arc<RemoteService>,
}

impl AppState {
    fn new(local: LocalService, remote: RemoteService) -> Self {
        let local_state = Arc::new(local);
        let remote_state = Arc::new(remote);
        let current_state = Arc::clone(&local_state);

        Self {
            current_service: current_state,
            local_service: local_state,
            remote_service: remote_state,
        }
    }

    fn current_state(&self) -> &dyn AppService {
        &*self.current_service
    }
}

impl NetworkListener for AppState {
    fn notify(&mut self, mode: NetworkMode) {
        self.current_service = match mode {
            NetworkMode::Local => self.local_service.clone(),
            NetworkMode::Remote => self.remote_service.clone(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let local_service = LocalService::new(app);
            let remote_service = RemoteService::new();

            let state = AppState::new(local_service, remote_service);

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
