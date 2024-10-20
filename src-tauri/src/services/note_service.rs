use crate::{
    http::{HttpClient, HttpClientImpl},
    models::note::Note,
};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tauri::Wry;
use tauri_plugin_store::Store;

use super::Service;

pub type NoteService = dyn Service<Note>;

pub struct LocalNoteService {
    store: Store<Wry>,
}

impl LocalNoteService {
    pub fn new(store: Store<Wry>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Service<Note> for LocalNoteService {
    async fn index(&self) -> Result<Vec<Note>> {
        let values: Vec<Note> = self
            .store
            .values()
            .iter()
            .map(|value| serde_json::from_value::<Note>(value.to_owned()).map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<Note>, _>>()?;

        Ok(values)
    }

    async fn show(&self, id: u32) -> Result<Note> {
        match self.store.get(id.to_string()) {
            Some(note) => Ok(serde_json::from_value(note)?),
            None => Err(anyhow!("Note {} not found", id)),
        }
    }

    async fn store(&self, param: Note) -> Result<()> {
        self.store
            .set(param.id().to_string(), serde_json::to_string(&param)?);

        Ok(())
    }

    async fn update(&self, param: Note) -> Result<()> {
        self.store
            .set(param.id().to_string(), serde_json::to_string(&param)?);

        Ok(())
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        match self.store.delete(id.to_string()) {
            true => Ok(()),
            false => Err(anyhow!("Could not delete note {}", id)),
        }
    }
}

#[derive(Debug)]
pub struct RemoteNoteService {
    client: HttpClientImpl, // TODO: use HttpClient trait
    slug: String,
}

impl RemoteNoteService {
    pub fn new(slug: String, client: HttpClientImpl) -> Self {
        Self { client, slug }
    }
}

#[async_trait]
impl Service<Note> for RemoteNoteService {
    async fn index(&self) -> Result<Vec<Note>> {
        let notes = self.client.get::<Vec<Note>>(&self.slug).await?;

        Ok(notes)
    }

    async fn show(&self, id: u32) -> Result<Note> {
        let url = format!("{}/{}", self.slug, id);

        let note = self.client.get::<Note>(&url).await?;

        Ok(note)
    }

    async fn update(&self, param: Note) -> Result<()> {
        unimplemented!("update {:?}", param)
    }

    async fn store(&self, param: Note) -> Result<()> {
        unimplemented!("store {:?}", param)
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        unimplemented!("destroy {:?}", id)
    }
}
