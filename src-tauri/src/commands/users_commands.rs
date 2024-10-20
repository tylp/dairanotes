use tauri::{Manager, State};

use crate::{models::user::User, AppState};

#[tauri::command]
pub async fn users_index(app_handle: tauri::AppHandle) -> Result<Vec<User>, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.services().user_service().index().await {
        Ok(user) => Ok(user),
        Err(_) => Err("failed to list users".to_string()),
    }
}

#[tauri::command]
pub async fn users_show(app_handle: tauri::AppHandle, id: u32) -> Result<User, String> {
    let state: State<'_, AppState> = app_handle.state();

    match state.services().user_service().show(id).await {
        Ok(user) => Ok(user),
        Err(_) => Err("note not found".to_string()),
    }
}
