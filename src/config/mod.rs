use serde::Deserialize;
use crate::folder::Folder;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub folders: Vec<FolderConfig>,
    pub api_key: String,
    pub idle_color: String,
    pub scanning_color: String,
    pub error_color: String,
    pub paused_color: String,
    pub refresh_millis: u64,
    pub dynamic_refresh_millis: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FolderConfig {
    pub id: String,
    pub icon: String,
}

use std::{fs, path::PathBuf};
use dirs::config_dir;
use toml;

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    // Resolve: ~/.config/ontake/lights-in-sync/config.toml
    let mut path: PathBuf = config_dir()
        .ok_or("No config directory found")?;

    path.push("ontake/lights-in-sync/config.toml");

    let content = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub fn load_folders_from_config(config: &Config) -> Result<Vec<Folder>, Box<dyn std::error::Error>> {
    let folders_list = config.clone()
        .folders
        .into_iter()
        .map(|f| Folder::new(f.id, f.icon))
        .collect();

    Ok(folders_list)
}
