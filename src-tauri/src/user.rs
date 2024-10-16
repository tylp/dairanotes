use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use user_service::UserService;

use crate::service::Service;

pub mod user_service;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct User {
    id: u32,
    username: String,
}

pub struct UserManager {
    user_service: Box<UserService>,
}

impl UserManager {
    pub fn new(user_service: Box<UserService>) -> Self {
        Self { user_service }
    }
}

#[async_trait]
impl Service<User> for UserManager {
    async fn index(&self) -> Result<Vec<User>> {
        self.user_service.index().await
    }

    async fn show(&self, id: u32) -> Result<User> {
        self.user_service.show(id).await
    }

    async fn store(&self, param: User) -> Result<()> {
        self.user_service.store(param).await
    }

    async fn update(&self, param: User) -> Result<()> {
        self.user_service.update(param).await
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        self.user_service.destroy(id).await
    }
}
