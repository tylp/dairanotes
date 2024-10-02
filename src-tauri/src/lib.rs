// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use serde::Serialize;

#[derive(Serialize)]
struct Note {
    id: u32,
    title: String,
}

#[tauri::command]
fn notes_index() -> Vec<Note> {
    vec![
        Note { id: 1, title: "First note".into() },
        Note { id: 2, title: "Second note".into() },
    ]
}

#[tauri::command]
fn notes_show(id: u32) -> Option<Note> {
    let notes = vec![
        Note { id: 1, title: "First note".into() },
        Note { id: 2, title: "Second note".into() },
    ];

    notes.into_iter().find(|note| note.id == id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![notes_index])
        .invoke_handler(tauri::generate_handler![notes_show])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
