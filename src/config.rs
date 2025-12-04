use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;
use crate::error::DownloaderError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub hostname: String,
    pub username: String,
    pub ssh_key_path: String,
    pub port: u16,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub servers: HashMap<String, ServerConfig>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Ok(Config {
                servers: HashMap::new(),
            });
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| DownloaderError::ConfigError(
                format!("Failed to read config file: {}", e)
            ))?;

        let config: Config = serde_json::from_str(&content)
            .map_err(|e| DownloaderError::ConfigError(
                format!("Failed to parse config file: {}", e)
            ))?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| DownloaderError::ConfigError(
                    format!("Failed to create config directory: {}", e)
                ))?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| DownloaderError::ConfigError(
                format!("Failed to serialize config: {}", e)
            ))?;

        fs::write(&config_path, content)
            .map_err(|e| DownloaderError::ConfigError(
                format!("Failed to write config file: {}", e)
            ))?;

        Ok(())
    }

    pub fn add_server(&mut self, name: String, config: ServerConfig) -> Result<()> {
        self.servers.insert(name, config);
        Ok(())
    }

    pub fn remove_server(&mut self, name: &str) -> Result<()> {
        self.servers.remove(name)
            .ok_or_else(|| DownloaderError::ServerNotFound(name.to_string()))?;
        Ok(())
    }

    pub fn get_server(&self, name: &str) -> Result<&ServerConfig> {
        Ok(self.servers.get(name)
            .ok_or_else(|| DownloaderError::ServerNotFound(name.to_string()))?)
    }

    pub fn list_servers(&self) -> Vec<(&String, &ServerConfig)> {
        self.servers.iter().collect()
    }

    fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or(DownloaderError::HomeDirectoryNotFound)?;

        Ok(home_dir.join(".downloader-from-server").join("config.json"))
    }
}