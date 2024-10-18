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
