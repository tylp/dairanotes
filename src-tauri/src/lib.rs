use std::sync::Arc;

use async_trait::async_trait;
use http::HttpClientImpl;
use note::note_service::NoteService;
use tauri::{Builder, Manager};
use tauri_plugin_store::StoreExt;

mod commands;
mod http;
mod note;
mod service;
mod user;

use note::note_service::LocalNoteService;
use note::note_service::RemoteNoteService;
use user::user_service::LocalUserService;
use user::user_service::RemoteUserService;
use user::user_service::UserService;

#[async_trait]
trait AppService: Send + Sync {
    fn note_service(&self) -> &NoteService;
    fn user_service(&self) -> &UserService;
}

struct LocalService {
    note_service: LocalNoteService,
    user_service: LocalUserService,
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
            let notes_store = app.handle().store_builder("notes.bin").build();
            let local_note_service = LocalNoteService::new(notes_store);
            let remote_note_service =
                RemoteNoteService::new("/notes".to_string(), HttpClientImpl::default());

            let user_store = app.handle().store_builder("users.bin").build();
            let local_user_service = LocalUserService::new(user_store);
            let remote_user_service =
                RemoteUserService::new("/users".to_string(), HttpClientImpl::default());

            let local_state = LocalService {
                note_service: local_note_service,
                user_service: local_user_service,
            };

            let remote_state = RemoteService {
                note_service: remote_note_service,
                user_service: remote_user_service,
            };

            let state = AppState::new(local_state, remote_state);

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
