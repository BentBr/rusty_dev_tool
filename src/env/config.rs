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
    pub local_key: String
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
            local_key: String::new(),
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
        local_key: String::new(),
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
        local_key: local_config.local_key,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_commands() {
        let mut home_commands = HashMap::new();
        home_commands.insert(
            String::from("home_command"),
            Command {
                command: String::from("echo Home"),
                alias: String::from("home"),
            },
        );

        let mut local_commands = HashMap::new();
        local_commands.insert(
            String::from("local_command"),
            Command {
                command: String::from("echo Local"),
                alias: String::from("local"),
            },
        );

        let merged_commands = merge_commands(home_commands, local_commands);

        assert_eq!(merged_commands.len(), 2);
        assert!(merged_commands.contains_key("home_command"));
        assert!(merged_commands.contains_key("local_command"));
    }

    #[test]
    fn test_merge_commands_override() {
        let mut home_commands = HashMap::new();
        home_commands.insert(
            String::from("home_command"),
            Command {
                command: String::from("echo Home"),
                alias: String::from("home"),
            },
        );

        home_commands.insert(
            String::from("home_command2"),
            Command {
                command: String::from("echo Home2"),
                alias: String::from("home"),
            },
        );

        let mut local_commands = HashMap::new();
        local_commands.insert(
            String::from("home_command2"),
            Command {
                command: String::from("overridden"),
                alias: String::from("moin"),
            },
        );

        let merged_commands = merge_commands(home_commands, local_commands);

        assert_eq!(merged_commands.len(), 2);
        assert!(merged_commands.contains_key("home_command"));
        assert!(merged_commands.contains_key("home_command2"));
        assert_eq!(
            merged_commands.get("home_command2").unwrap().command,
            "overridden"
        );
    }

    #[test]
    fn test_merge_configs_with_local() {
        let home_config = HomeConfig::default();
        let local_config = LocalConfig {
            commands: HashMap::new(),
            local_key: String::from("new-project"),
            environments: HashMap::new(),
        };
        let compose = Compose::DockerCompose;
        let language_framework = LanguageFramework::Rust;

        let config = merge_configs_with_local(home_config, local_config, compose, language_framework);

        assert_eq!(config.rdt_name, "rdt");
        assert_eq!(config.download_path, "https://github.com/BentBr/rusty_dev_tool/releases/download");
        assert_eq!(config.meta_path, "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest");
        assert!(config.commands.is_empty());
        assert!(config.environments.is_empty());
        assert_eq!(config.compose, Compose::DockerCompose);
        assert_eq!(config.language_framework, LanguageFramework::Rust);
        assert_eq!(config.local_key, "new-project");
    }

    #[test]
    fn test_get_config_without_local() {
        let home_config = HomeConfig::default();

        let config = get_config_without_local(home_config);

        assert_eq!(config.rdt_name, "rdt");
        assert_eq!(config.download_path, "https://github.com/BentBr/rusty_dev_tool/releases/download");
        assert_eq!(config.meta_path, "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest");
        assert!(config.commands.is_empty());
        assert!(config.environments.is_empty());
        assert_eq!(config.local_key, "");
    }
}
