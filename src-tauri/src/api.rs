use anyhow::Result;
use serde::de::DeserializeOwned;
use tauri_plugin_http::reqwest;

/// Struct that wrap the HTTP library.
#[derive(Debug, Default)]
pub struct HttpClient {
    client: reqwest::Client,
}

impl HttpClient {
    /// Send a GET request to the given URL and return the reponse as T.
    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.client.get(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(response.json::<T>().await?),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    /// Send a PATCH request to the given URL with the given parameters.
    pub async fn patch(&self, url: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let response = self.client.patch(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    /// Send a POST request to the given URL with the given parameters.
    pub async fn post(&self, url: &str, params: Vec<(&str, &str)>) -> Result<()> {
        let response = self.client.post(url).json(&params).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }

    /// Send a DELETE request to the given URL.
    pub async fn delete(&self, url: &str) -> Result<()> {
        let response = self.client.delete(url).send().await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            status => Err(anyhow::anyhow!("unexpected status code: {}", status)),
        }
    }
}
