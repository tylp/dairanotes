use tauri::{Manager, State};

use crate::{note::Note, service::Service, user::User, AppState};

#[tauri::command]
pub async fn notes_index(app_handle: tauri::AppHandle) -> Result<Vec<Note>, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.notes_manager().index().await {
        Ok(notes) => Ok(notes),
        Err(_) => Err("failed to list notes".to_string()),
    }
}

#[tauri::command]
pub async fn notes_show(app_handle: tauri::AppHandle, id: u32) -> Result<Note, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.notes_manager().show(id).await {
        Ok(note) => Ok(note),
        Err(_) => Err("note not found".to_string()),
    }
}

#[tauri::command]
pub async fn user_index(app_handle: tauri::AppHandle) -> Result<Vec<User>, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.user_manager().index().await {
        Ok(user) => Ok(user),
        Err(_) => Err("failed to list users".to_string()),
    }
}

#[tauri::command]
pub async fn user_show(app_handle: tauri::AppHandle, id: u32) -> Result<User, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.user_manager().show(id).await {
        Ok(user) => Ok(user),
        Err(_) => Err("note not found".to_string()),
    }
}
