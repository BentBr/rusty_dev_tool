use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    TomlNotReadable(String, String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TomlNotReadable(dir, error) => {
                write!(
                    f,
                    "Could not read your toml in dir '{dir}' with error: {error}",
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
        let error = Error::TomlNotReadable("test_dir".to_string(), "test_error".to_string());
        assert_eq!(
            format!("{}", error),
            "Could not read your toml in dir 'test_dir' with error: test_error"
        );
    }
}
