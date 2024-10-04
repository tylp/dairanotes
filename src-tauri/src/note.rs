use std::sync::Arc;

use note_api::NoteApi;
use serde::{Deserialize, Serialize};

use crate::api::HttpClient;

mod note_api;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Note {
    id: u32,
    title: String,
}

impl Note {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

/// Handle the API to communicate with the server.
/// Stores a local list of the notes.
#[derive(Debug)]
pub struct NoteManager {
    http_client: Arc<HttpClient>,
    notes_api: NoteApi,
    notes: Vec<Note>,
}

impl Default for NoteManager {
    fn default() -> Self {
        let root = "http://localhost:8000".to_string();
        let slug = "/notes".to_string();

        let http_client = Arc::new(HttpClient::new(root));
        let note_api = NoteApi::new(slug, Arc::clone(&http_client));

        Self {
            notes_api: note_api,
            http_client,
            notes: vec![],
        }
    }
}

impl NoteManager {
    /// Return all the local notes.
    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    /// Return a local note by its id.
    pub fn get_note(&self, id: u32) -> Option<Note> {
        self.notes.iter().find(|note| note.id() == id).cloned()
    }
}
