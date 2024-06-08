use crate::env::config::{Command, PathOptions, DEFAULT_CONFIG_FILE};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LocalConfig {
    pub commands: HashMap<String, Command>,
    pub local_key: String,
    pub environments: HashMap<String, Environment>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Environment {
    pub name: String,
    pub uri: String,
}

impl LocalConfig {
    fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let config: LocalConfig = toml::from_str(&content)?;

        Ok(config)
    }
}

pub fn get_local_config() -> Result<Option<LocalConfig>, Box<dyn Error>> {
    // We explicitly allow the local config to be missing
    let local_config_dir = match PathOptions::new().get_local_dir_default() {
        Ok(dir) => dir,
        Err(_) => return Ok(None),
    };

    let config_file = local_config_dir.join(DEFAULT_CONFIG_FILE.as_str());
    if config_file.exists() {
        return LocalConfig::from_file(config_file.to_string_lossy().to_string().as_str())
            .map(Some);
    }

    Ok(None)
}
