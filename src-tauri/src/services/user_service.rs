use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tauri::Wry;
use tauri_plugin_store::Store;

use crate::{
    models::user::User,
    utils::http::{HttpClient, HttpClientImpl},
};

use super::Service;

pub type UserService = dyn Service<User>;

pub struct LocalUserService {
    store: Store<Wry>,
}

impl LocalUserService {
    pub fn new(store: Store<Wry>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl Service<User> for LocalUserService {
    async fn index(&self) -> Result<Vec<User>> {
        let values: Vec<User> = self
            .store
            .values()
            .iter()
            .map(|value| serde_json::from_value::<User>(value.to_owned()).map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<User>, _>>()?;

        Ok(values)
    }

    async fn show(&self, id: u32) -> Result<User> {
        match self.store.get(id.to_string()) {
            Some(user) => Ok(serde_json::from_value(user)?),
            None => Err(anyhow!("User {} not found", id)),
        }
    }

    async fn store(&self, param: User) -> Result<()> {
        self.store
            .set(param.id().to_string(), serde_json::to_string(&param)?);

        Ok(())
    }

    async fn update(&self, param: User) -> Result<()> {
        self.store
            .set(param.id().to_string(), serde_json::to_string(&param)?);

        Ok(())
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        match self.store.delete(id.to_string()) {
            true => Ok(()),
            false => Err(anyhow!("Could not delete user {}", id)),
        }
    }
}

#[derive(Debug)]
pub struct RemoteUserService {
    client: HttpClientImpl,
    slug: String,
}

impl RemoteUserService {
    pub fn new(slug: String, client: HttpClientImpl) -> Self {
        Self { client, slug }
    }
}

#[async_trait]
impl Service<User> for RemoteUserService {
    async fn index(&self) -> Result<Vec<User>> {
        let users = self.client.get::<Vec<User>>(&self.slug).await?;

        Ok(users)
    }

    async fn show(&self, id: u32) -> Result<User> {
        let url = format!("{}/{}", self.slug, id);

        let user = self.client.get::<User>(&url).await?;

        Ok(user)
    }

    async fn update(&self, param: User) -> Result<()> {
        unimplemented!("update {:?}", param)
    }

    async fn store(&self, param: User) -> Result<()> {
        unimplemented!("store {:?}", param)
    }

    async fn destroy(&self, id: u32) -> Result<()> {
        unimplemented!("destroy {:?}", id)
    }
}
