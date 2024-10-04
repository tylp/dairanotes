use note_api::NoteApi;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Default)]
pub struct NoteManager {
    notes_api: NoteApi,
    notes: Vec<Note>,
}

impl NoteManager {
    /// Return all the local notes.
    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    /// Return a note by its id.
    pub fn get_note(&self, id: u32) -> Option<Note> {
        self.notes.iter().find(|note| note.id() == id).cloned()
    }
}
