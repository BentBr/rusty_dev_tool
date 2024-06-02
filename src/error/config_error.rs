use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConfigError {
    TomlNotReadable(String),
    TomlMissingNode(String),
}

impl Error for ConfigError {}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::TomlNotReadable(dir) => {
                write!(f, "Could not read your toml env in dir '{}'", dir)
            }
            ConfigError::TomlMissingNode(node) => {
                write!(f, "Your toml config is missing the node '{}'", node)
            }
        }
    }
}
