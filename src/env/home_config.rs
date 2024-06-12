use crate::env::config::{Command, PathOptions, DEFAULT_CONFIG_FILE};
use crate::env::traits::FromFile;
use crate::error::config::Error as ConfigError;
use crate::error::file_system::Error as FileSystemError;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct HomeConfig {
    pub rdt_name: String,
    pub download_path: String,
    pub meta_path: String,
    pub commands: HashMap<String, Command>,
}

impl HomeConfig {
    // This is our default config being written to home config file when no one is found during startup
    fn default() -> Self {
        Self {
            rdt_name: String::from("rdt"),
            download_path: String::from(
                "https://github.com/BentBr/rusty_dev_tool/releases/download",
            ),
            meta_path: String::from(
                "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest",
            ),
            commands: HashMap::new(),
        }
    }
}

impl FromFile for HomeConfig {}

pub fn get_or_create(restore: bool) -> Result<HomeConfig, Box<dyn Error>> {
    let home_config_dir = PathOptions::new().get_or_create_home_dir_default()?;

    let config_file = home_config_dir.join(DEFAULT_CONFIG_FILE.as_str());

    if !restore && config_file.exists() {
        return Ok(
            HomeConfig::from_file(config_file.to_string_lossy().to_string().as_str()).map_err(
                |error| {
                    ConfigError::TomlNotReadable(
                        config_file.to_string_lossy().to_string(),
                        error.to_string(),
                    )
                },
            )?,
        );
    }

    create_new_home_config(config_file.as_path())
}

fn create_new_home_config(config_file_path: &Path) -> Result<HomeConfig, Box<dyn Error>> {
    if config_file_path.exists() {
        backup_home_config(config_file_path)?;
    }

    println!();

    println!(
        "Creating new home config file at: {}",
        config_file_path.to_string_lossy().blue()
    );

    let default_config = HomeConfig::default();
    let toml_str = toml::to_string(&default_config)?;

    fs::write(config_file_path, toml_str)
        .map_err(|error| FileSystemError::FileWriteFailed(error.to_string()))?;

    Ok(default_config)
}

fn backup_home_config(config_file_path: &Path) -> Result<(), Box<dyn Error>> {
    let backup_file_path = config_file_path.with_extension("bak");
    fs::copy(config_file_path, backup_file_path.clone())?;

    println!();

    println!(
        "Backing up home config file at: {} to {}",
        config_file_path.to_string_lossy().blue(),
        backup_file_path.to_string_lossy().blue()
    );

    Ok(())
}
