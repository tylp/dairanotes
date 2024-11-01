use anyhow::Result;
use async_trait::async_trait;

pub mod network_service;
pub mod note_service;
pub mod user_service;

#[async_trait]
pub trait Service<T>: Send + Sync {
    async fn index(&self) -> Result<Vec<T>>;
    async fn show(&self, id: u32) -> Result<T>;
    async fn store(&self, param: T) -> Result<()>;
    async fn update(&self, param: T) -> Result<()>;
    async fn destroy(&self, id: u32) -> Result<()>;
}
