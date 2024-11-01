use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};
use tauri::{App, Manager};

pub static CONFIGURATION: LazyLock<Mutex<Configuration>> =
    LazyLock::new(|| Mutex::new(Configuration::default()));

pub fn load_configuration(app: &mut App) -> Result<()> {
    let resource_path = app
        .path()
        .resolve("config.json", tauri::path::BaseDirectory::Resource)?;
    let file = std::fs::File::open(&resource_path).expect("failed to open config file");
    let config: Configuration = serde_json::from_reader(file).expect("failed to parse config");
    *CONFIGURATION.lock().unwrap() = config;
    Ok(())
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    network: NetworkConfiguration,
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    server: String,
    health_check_interval: u16,
}

impl Configuration {
    pub fn network(&self) -> &NetworkConfiguration {
        &self.network
    }
}

impl NetworkConfiguration {
    pub fn server(&self) -> &String {
        &self.server
    }

    pub fn health_check_interval(&self) -> u16 {
        self.health_check_interval
    }
}
