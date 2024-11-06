use std::{fmt::Display, sync::Arc, time::Duration};

use async_trait::async_trait;
use tauri::async_runtime::Mutex;
use tauri_plugin_http::reqwest;

use crate::config;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum FrontEndEvent {
    NetworkMode,
}

impl Display for FrontEndEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontEndEvent::NetworkMode => write!(f, "NetworkMode"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum NetworkMode {
    Local,
    Remote,
}

impl Display for NetworkMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkMode::Local => write!(f, "Local"),
            NetworkMode::Remote => write!(f, "Remote"),
        }
    }
}

#[async_trait]
pub trait NetworkListener: Send + Sync {
    fn notify(&mut self, mode: NetworkMode);
}

pub struct NetworkMonitor {
    listeners: Vec<Arc<Mutex<dyn NetworkListener>>>,
}

impl NetworkMonitor {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn add_listener(&mut self, listener: Arc<Mutex<dyn NetworkListener>>) {
        self.listeners.push(listener);
    }

    pub fn monitor(&self) {
        let listeners = self.listeners.clone();
        let config = config::CONFIGURATION.lock().unwrap();
        let url = format!("http://{}/ping", config.network().server());
        let tick = Duration::from_secs(u64::from(config.network().health_check_interval()));
        tauri::async_runtime::spawn(async move {
            let mut previous_mode: Option<NetworkMode> = None;
            let client = reqwest::Client::new();

            loop {
                let url = url.clone();
                match client.get(url).send().await {
                    Ok(response) if response.status() == reqwest::StatusCode::OK => {
                        if previous_mode != Some(NetworkMode::Remote) {
                            previous_mode = Some(NetworkMode::Remote);
                            for listener in listeners.iter() {
                                let mut listener = listener.lock().await;
                                listener.notify(NetworkMode::Remote);
                            }
                        }
                    }
                    _ => {
                        if previous_mode != Some(NetworkMode::Local) {
                            previous_mode = Some(NetworkMode::Local);
                            for listener in listeners.iter() {
                                let mut listener = listener.lock().await;
                                listener.notify(NetworkMode::Local);
                            }
                        }
                    }
                }

                tokio::time::sleep(tick).await;
            }
        });
    }
}
