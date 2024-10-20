use async_trait::async_trait;
use services::{
    note_service::{LocalNoteService, NoteService, RemoteNoteService},
    user_service::{LocalUserService, RemoteUserService, UserService},
};
use tauri::{App, Builder, Manager};
use tauri_plugin_store::StoreExt;
use utils::http::HttpClientImpl;

mod commands;
mod models;
mod services;
mod utils;

#[async_trait]
trait State: Send + Sync {
    fn note_service(&self) -> &NoteService;
    fn user_service(&self) -> &UserService;
}

struct LocalState {
    note_service: LocalNoteService,
    user_service: LocalUserService,
}

impl LocalState {
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
impl State for LocalState {
    fn note_service(&self) -> &NoteService {
        &self.note_service
    }

    fn user_service(&self) -> &UserService {
        &self.user_service
    }
}

struct RemoteState {
    note_service: RemoteNoteService,
    user_service: RemoteUserService,
}

impl RemoteState {
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
impl State for RemoteState {
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
    network_mode: NetworkMode,
    local_service: LocalState,
    remote_service: RemoteState,
}

impl AppState {
    fn new(local: LocalState, remote: RemoteState) -> Self {
        let local_state = local;
        let remote_state = remote;

        Self {
            network_mode: NetworkMode::Local,
            local_service: local_state,
            remote_service: remote_state,
        }
    }

    fn current_state(&self) -> &dyn State {
        match self.network_mode {
            NetworkMode::Local => &self.local_service,
            NetworkMode::Remote => &self.remote_service,
        }
    }
}

impl NetworkListener for AppState {
    fn notify(&mut self, mode: NetworkMode) {
        self.network_mode = mode;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let local_state = LocalState::new(app);
            let remote_state = RemoteState::new();

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
