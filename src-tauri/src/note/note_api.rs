use std::sync::Arc;

use crate::api::HttpClient;
use anyhow::Result;

use super::Note;

/// Handler to communicate with the server through the notes API.
#[derive(Debug)]
pub struct NoteApi {
    client: Arc<HttpClient>,
    slug: String,
}

impl NoteApi {
    pub fn new(slug: String, client: Arc<HttpClient>) -> Self {
        Self { client, slug }
    }

    pub async fn get_note(&self, id: u32) -> Result<Note> {
        let url = format!("{}/{}", self.slug, id);

        let note = self.client.get::<Note>(&url).await?;

        Ok(note)
    }

    pub async fn get_notes(&self) -> Result<Vec<Note>> {
        let notes = self.client.get::<Vec<Note>>(&self.slug).await?;

        Ok(notes)
    }
}
