use crate::api::HttpClient;
use anyhow::Result;

use super::Note;

/// Handler to communicate with the server through the notes API.
#[derive(Debug)]
pub struct NoteApi {
    client: HttpClient,
    base_url: String,
}

impl Default for NoteApi {
    fn default() -> Self {
        Self {
            client: HttpClient::default(),
            base_url: "http://localhost:3000/notes".to_string(),
        }
    }
}

impl NoteApi {
    pub fn new(base_url: String) -> Self {
        Self {
            client: HttpClient::default(),
            base_url: format!("{}/notes", base_url),
        }
    }

    /// Fetch a note by its id.
    pub async fn get_note(&self, id: u32) -> Result<Note> {
        let url = format!("{}/{}", self.base_url, id);

        let note = self.client.get::<Note>(&url).await?;

        Ok(note)
    }

    /// Fetch all the notes.
    pub async fn get_notes(&self) -> Result<Vec<Note>> {
        let notes = self.client.get::<Vec<Note>>(&self.base_url).await?;

        Ok(notes)
    }
}
