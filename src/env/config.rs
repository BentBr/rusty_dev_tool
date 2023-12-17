use crate::error::file_system_error::FileSystemError;
use lazy_static::lazy_static;
use std::fs;

use dirs::home_dir;
use fs::create_dir;

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

            println!("the home dir: {:?}", home.canonicalize());

            if home.exists() && home.to_str().is_some() {
                if let Some(dir_str) = home.to_str() {
                    return Ok(dir_str.to_string());
                }
            }

            // Creating the dir

            create_dir(&home).map_err(|e| FileSystemError::FileNotFound(e.to_string()))?;
            return Ok(home.to_string_lossy().into_owned());
        }

        Err(FileSystemError::FileNotFound(String::from(
            "Home dir of current user",
        )))
    }
}

lazy_static! {
    pub static ref DEFAULT_FOLDER: String = String::from(".rusty_dev_tool");
}
