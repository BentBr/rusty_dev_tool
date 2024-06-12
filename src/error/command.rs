use crate::error::update::Error as UpdateError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
    ExecutionFailedGeneric(Box<dyn StdError>),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandNotFound(dir) => write!(
                f,
                "Command not found: '{dir}'. Maybe create a custom one in config?"
            ),
            Self::ExecutionFailedGeneric(error) => {
                write!(f, "Command execution failed: '{error}'")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::ExecutionFailedGeneric(Box::new(error))
    }
}

impl From<UpdateError> for Error {
    fn from(error: UpdateError) -> Self {
        Self::ExecutionFailedGeneric(Box::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_not_found_error() {
        let error = Error::CommandNotFound("test_command".to_string());
        assert_eq!(
            format!("{}", error),
            "Command not found: 'test_command'. Maybe create a custom one in config?"
        );
    }

    #[test]
    fn test_execution_failed_generic_error() {
        let error = Error::ExecutionFailedGeneric(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test_error",
        )));
        assert_eq!(
            format!("{}", error),
            "Command execution failed: 'test_error'"
        );
    }
}
