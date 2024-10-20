use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest;

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get<T: DeserializeOwned>(&self, slug: &str) -> Result<T>;
    async fn patch(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()>;
    async fn post(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()>;
    async fn delete(&self, slug: &str) -> Result<()>;
}

#[derive(Debug)]
pub struct HttpClientImpl {
    client: reqwest::Client,
    root: String,
}

impl Default for HttpClientImpl {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            root: "http://localhost:8000".to_string(),
        }
    }
}

#[async_trait]
impl HttpClient for HttpClientImpl {
    async fn get<T: DeserializeOwned>(&self, slug: &str) -> Result<T> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    async fn patch(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.patch(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    async fn post(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.post(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    async fn delete(&self, slug: &str) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.delete(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }
}
