use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    id: u32,
    username: String,
}

impl User {
    pub fn new(id: u32, username: String) -> Self {
        Self { id, username }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}
