use tauri::{Manager, State};

use crate::{note::Note, AppState};

#[tauri::command]
pub async fn notes_index(app_handle: tauri::AppHandle) -> Result<Vec<Note>, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.current_state().note_service().index().await {
        Ok(notes) => Ok(notes),
        Err(_) => Err("failed to list notes".to_string()),
    }
}

#[tauri::command]
pub async fn notes_show(app_handle: tauri::AppHandle, id: u32) -> Result<Note, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.current_state().note_service().show(id).await {
        Ok(note) => Ok(note),
        Err(_) => Err("note not found".to_string()),
    }
}
