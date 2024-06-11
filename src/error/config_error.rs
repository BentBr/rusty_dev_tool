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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toml_not_readable_error() {
        let error = ConfigError::TomlNotReadable("test_dir".to_string(), "test_error".to_string());
        assert_eq!(
            format!("{}", error),
            "Could not read your toml in dir 'test_dir' with error: test_error"
        );
    }
}
