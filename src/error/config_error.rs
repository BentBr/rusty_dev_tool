use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    TomlNotReadable(String, String),
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::TomlNotReadable(dir, error) => {
                write!(
                    f,
                    "Could not read your toml in dir '{}' with error: {}",
                    dir, error
                )
            }
        }
    }
}