use crate::error::file_system_error::FileSystemError;
use lazy_static::lazy_static;
use std::{env, fs};

use dirs::home_dir;
use fs::create_dir;
use std::path::PathBuf;

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

    pub fn with_option(mut self, default_path: String) -> Self {
        self.default_path = default_path;
        self
    }

    pub fn get_or_create_home_dir(&self) -> Result<String, FileSystemError> {
        if let Some(mut home) = home_dir() {
            // Append your tool's configuration directory to the home directory
            home.push(self.default_path.as_str());

            return find_or_create_default_dir(home);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Home dir of current user",
        )))
    }

    pub fn get_or_create_project_dir(&self) -> Result<String, FileSystemError> {
        if let Ok(mut current_dir) = env::current_dir() {
            // Append your tool's configuration directory to the home directory
            current_dir.push(self.default_path.as_str());

            return find_or_create_default_dir(current_dir);
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Current project dir",
        )))
    }
}

lazy_static! {
    pub static ref DEFAULT_FOLDER: String = String::from(".rusty_dev_tool");
}

fn find_or_create_default_dir(path: PathBuf) -> Result<String, FileSystemError> {
    // If the default dir already exists just return it
    if path.exists() && path.to_str().is_some() {
        if let Some(dir_str) = path.to_str() {
            return Ok(dir_str.to_string());
        }
    }

    // Creating the dir
    create_dir(&path).map_err(|e| FileSystemError::FolderNotFound(e.to_string()))?;

    return Ok(path.to_string_lossy().into_owned());
}
