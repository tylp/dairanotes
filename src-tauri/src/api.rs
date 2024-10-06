use anyhow::Result;
use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest;

#[derive(Debug)]
pub struct HttpClient {
    client: reqwest::Client,
    root: String,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
            root: "http://localhost:8000".to_string(),
        }
    }
}

impl HttpClient {
    pub fn new(root: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            root,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, slug: &str) -> Result<T> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    pub async fn patch(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.patch(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    pub async fn post(&self, slug: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.post(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    pub async fn delete(&self, slug: &str) -> Result<()> {
        let url = format!("{}/{}", self.root, slug);
        let response = self.client.delete(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }
}
