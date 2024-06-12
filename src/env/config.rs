use crate::env::compose::r#enum::Compose;
use crate::env::home_config::HomeConfig;
use crate::env::language::r#enum::LanguageFramework;
use crate::env::local_config::{Environment, LocalConfig};
use crate::error::file_system::Error as FileSystemError;
use dirs::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;

lazy_static! {
    pub static ref DEFAULT_FOLDER: String = String::from(".rusty_dev_tool");
    pub static ref DEFAULT_CONFIG_FILE: String = String::from("config.toml");
}

/**
* Config struct that is the result of merged home and local project config
*/
#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    pub rdt_name: String,
    pub download_path: String,
    pub meta_path: String,
    pub commands: HashMap<String, Command>,
    pub environments: HashMap<String, Environment>,
    pub compose: Compose,
    pub language_framework: LanguageFramework,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Command {
    pub command: String,
    pub alias: String,
}

pub struct PathOptions {
    default_path: String,
}

impl PathOptions {
    pub fn new() -> Self {
        // Set default values here
        Self {
            default_path: DEFAULT_FOLDER.to_string(),
        }
    }

    pub fn get_or_create_home_dir_default(&self) -> Result<PathBuf, FileSystemError> {
        if let Some(mut home) = home_dir() {
            // Append your tool's configuration directory to the home directory
            home = home.join(self.default_path.as_str());

            return find_or_create_default_dir_in(home);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Home dir of current user",
        )))
    }

    pub fn get_local_dir_default_config(&self) -> Result<PathBuf, FileSystemError> {
        if let Ok(current_dir) = self.get_local_working_dir() {
            let local_dir_default = current_dir.join(self.default_path.as_str());

            if local_dir_default.exists() {
                return Ok(local_dir_default);
            }
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Local local default config dir",
        )))
    }

    #[allow(clippy::unused_self)]
    pub fn get_local_working_dir(&self) -> Result<PathBuf, FileSystemError> {
        if let Ok(current_dir) = env::current_dir() {
            return Ok(current_dir);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Local local dir",
        )))
    }
}

fn find_or_create_default_dir_in(mut path: PathBuf) -> Result<PathBuf, FileSystemError> {
    if path.exists() && path.is_dir() && path.ends_with(DEFAULT_FOLDER.as_str()) {
        return Ok(path);
    }

    if !path.ends_with(DEFAULT_FOLDER.as_str()) {
        path = path.join(DEFAULT_FOLDER.as_str());
    }

    create_dir_all(&path).map_err(|error| FileSystemError::FolderNotFound(error.to_string()))?;

    Ok(path)
}

pub fn merge_configs(
    home_config: HomeConfig,
    local_config: Option<LocalConfig>,
    compose: Compose,
    language_framework: LanguageFramework,
) -> Config {
    match local_config {
        Some(local_config) => {
            merge_configs_with_local(home_config, local_config, compose, language_framework)
        }
        None => Config {
            rdt_name: home_config.rdt_name,
            download_path: home_config.download_path,
            meta_path: home_config.meta_path,
            commands: home_config.commands,
            environments: HashMap::new(),
            compose,
            language_framework,
        },
    }
}

pub fn get_config_without_local(home_config: HomeConfig) -> Config {
    Config {
        rdt_name: home_config.rdt_name,
        download_path: home_config.download_path,
        meta_path: home_config.meta_path,
        commands: home_config.commands,
        environments: HashMap::new(),
        compose: Compose::DockerCompose,
        language_framework: LanguageFramework::Rust,
    }
}

fn merge_configs_with_local(
    home_config: HomeConfig,
    local_config: LocalConfig,
    compose: Compose,
    language_framework: LanguageFramework,
) -> Config {
    Config {
        rdt_name: home_config.rdt_name,
        download_path: home_config.download_path,
        meta_path: home_config.meta_path,
        commands: merge_commands(home_config.commands, local_config.commands),
        environments: local_config.environments,
        compose,
        language_framework,
    }
}

/**
* Merge home and local commands where the local ones always override the home ones
*/
fn merge_commands(
    home_commands: HashMap<String, Command>,
    local_commands: HashMap<String, Command>,
) -> HashMap<String, Command> {
    let mut merged_commands = home_commands;
    merged_commands.extend(local_commands);

    merged_commands
}
