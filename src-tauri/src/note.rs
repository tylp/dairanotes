use std::fmt::Debug;

use serde::{Deserialize, Serialize};

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
