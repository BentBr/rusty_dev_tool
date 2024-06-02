use crate::error::file_system_error::FileSystemError;
use lazy_static::lazy_static;
use std::env;

use dirs::home_dir;
use serde::Serialize;
use std::fs::{create_dir_all, File};
use std::io::Read;
use std::path::PathBuf;

pub struct PathOptions {
    default_path: String,
}

#[derive(Serialize)]
pub struct Config {
    misc: Misc,
    commands: Commands,
}

#[derive(Serialize)]

pub struct Misc {
    name: String,
}

#[derive(Serialize)]

pub struct Commands {
    commands: Vec<Command>,
}

#[derive(Serialize)]
pub struct Command {
    command: String,
    alias: String,
}

impl PathOptions {
    pub fn new() -> Self {
        // Set default values here
        Self {
            default_path: DEFAULT_FOLDER.to_string(),
        }
    }

    pub fn with_option(mut self, default_path: String) -> Self {
        self.default_path = default_path;
        self
    }

    pub fn get_or_create_home_dir_default(&self) -> Result<PathBuf, FileSystemError> {
        if let Some(mut home) = home_dir() {
            // Append your tool's configuration directory to the home directory
            home = home.join(self.default_path.as_str());

            return find_or_create_default_dir(home);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Home dir of current user",
        )))
    }

    pub fn get_or_create_project_dir_default(&self) -> Result<PathBuf, FileSystemError> {
        if let Ok(mut current_dir) = env::current_dir() {
            // Append your tool's configuration directory to the home directory
            // todo: remove
            println!("current dir during creation: {:?}", current_dir);
            current_dir = current_dir.join(self.default_path.as_str());
            println!(
                "current dir during creation with path for env: {:?}",
                current_dir
            );

            return find_or_create_default_dir(current_dir);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Current project dir",
        )))
    }
}

pub fn read_config(config_path: PathBuf) -> Result<String, FileSystemError> {
    if let Ok(mut config_file) = File::open(&config_path) {
        let mut content = String::new();
        config_file
            .read_to_string(&mut content)
            .expect(&*String::from(format!(
                "Could not read file: {}",
                &config_path.to_string_lossy()
            )));

        return Ok(content);
    }
    // Todo: remove
    println!("config: {:?}", config_path);
    Err(FileSystemError::AccessNotAllowed(String::from(format!(
        "Could not read file: {}",
        config_path.to_string_lossy()
    ))))
}

/*
pub fn parse_toml(toml_str: String) -> Result<Config, ConfigError> {

}
*/

lazy_static! {
    pub static ref DEFAULT_FOLDER: String = String::from(".rusty_dev_tool");
    pub static ref DEFAULT_ENV_FILE: String = String::from("config.toml");
}

fn find_or_create_default_dir(mut path: PathBuf) -> Result<PathBuf, FileSystemError> {
    // If the default dir already exists just return it
    println!(
        "ends with Default folder: {}",
        path.ends_with(DEFAULT_FOLDER.as_str())
    );
    if path.exists() && path.is_dir() && path.ends_with(DEFAULT_FOLDER.as_str()) {
        // todo: remove
        println!("path existed!");
        return Ok(path);
    }
    println!("Exists now: {}", path.exists());

    println!("path did not exist -> creating: {:?}", path);
    // Creating the dir
    path = path.join(DEFAULT_FOLDER.as_str());
    create_dir_all(&path).map_err(|error| FileSystemError::FolderNotFound(error.to_string()))?;
    println!("Exists now: {}", path.exists());
    println!(
        "ends with Default folder: {}",
        path.ends_with(DEFAULT_FOLDER.as_str())
    );

    return Ok(path.to_owned());
}
