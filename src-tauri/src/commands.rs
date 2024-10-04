use std::sync::Mutex;

use tauri::State;

use crate::{note::Note, AppState};

/// Return a string formated with the given name.
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// Return a list of notes.
#[tauri::command]
pub fn notes_index(state: State<'_, Mutex<AppState>>) -> Vec<Note> {
    let state = state.lock().expect("could not lock state");

    state.notes_manager().get_notes()
}

/// Return a note by its id.
#[tauri::command]
pub fn notes_show(state: State<'_, Mutex<AppState>>, id: u32) -> Option<Note> {
    let state = state.lock().expect("could not lock state");

    state.notes_manager().get_note(id)
}
