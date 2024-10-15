use std::fmt::Debug;

use anyhow::Result;
use async_trait::async_trait;
use note_service::NoteService;
use serde::{Deserialize, Serialize};

use crate::service::Service;

pub mod note_service;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Note {
    id: u32,
    title: String,
}

pub struct NoteManager {
    note_service: Box<NoteService>,
}

impl NoteManager {
    pub fn new(note_service: Box<NoteService>) -> Self {
        Self { note_service }
    }
}

#[async_trait]
impl Service<Note> for NoteManager {
    async fn index(&self) -> Result<Vec<Note>> {
        self.note_service.index().await
    }

    async fn show(&self, id: u32) -> Result<Note> {
        self.note_service.show(id).await
    }

    async fn store(&self, param: Note) -> Result<()> {
        self.note_service.store(param).await
    }

    async fn update(&self, param: Note) -> Result<()> {
        self.note_service.update(param).await
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        self.note_service.destroy(id).await
    }
}
