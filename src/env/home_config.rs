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
    pub fn default() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use env::temp_dir;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_from_file() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test4.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            rdt_name = "test_name"
            download_path = "short-path"
            meta_path = "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest"
            [commands]
                [commands.test_command]
                    command = "echo Hello, World!"
                    alias = "test"
            "#
        )
        .unwrap();

        let config = HomeConfig::from_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(config.rdt_name, "test_name");
        assert_eq!(config.download_path, "short-path");
        assert!(config.commands.contains_key("test_command"));
        assert!(config
            .commands
            .get("test_command")
            .unwrap()
            .command
            .eq("echo Hello, World!"));
        assert!(config
            .commands
            .get("test_command")
            .unwrap()
            .alias
            .eq("test"));
    }

    #[test]
    fn test_from_file_fail_commands() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test5.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            rdt_name = "test_name"
            download_path = "short-path"
            meta_path = "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest"
            "#
        )
        .unwrap();

        let config = HomeConfig::from_file(file_path.to_str().unwrap());

        assert!(config.is_err());
        assert!(config
            .unwrap_err()
            .to_string()
            .contains("missing field `commands`"));
    }

    #[test]
    fn test_from_file_fail_meta_path() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test6.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            rdt_name = "test_name"
            download_path = "short-path"
            metaaa_path = "https://api.github.com/repos/BentBr/rusty_dev_tool/releases/latest"
            [commands]
            "#
        )
        .unwrap();

        let config = HomeConfig::from_file(file_path.to_str().unwrap());

        assert!(config.is_err());
        assert!(config
            .unwrap_err()
            .to_string()
            .contains("missing field `meta_path`"));
    }

    #[test]
    fn test_get_or_create_fresh() {
        let dir = temp_dir();
        env::set_var("HOME", dir.as_path());

        let config = get_or_create(false).unwrap();

        assert_eq!(config.rdt_name, "rdt");
        assert_eq!(
            config.download_path,
            "https://github.com/BentBr/rusty_dev_tool/releases/download"
        );
        assert!(config.commands.is_empty());
    }
}
