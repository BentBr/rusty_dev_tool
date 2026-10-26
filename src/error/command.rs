use crate::error::environment::Error as EnvironmentError;
use crate::error::update::Error as UpdateError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
    CommandFailed(String),
    NonZeroExit(i32),
    ExecutionFailedGeneric(Box<dyn StdError>),
}

impl StdError for Error {}

impl Error {
    /// Process exit code to report for this error.
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::NonZeroExit(code) => *code,
            _ => 1,
        }
    }
}

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
            Self::CommandFailed(error) => write!(f, "Command failed: '{error}'"),
            Self::NonZeroExit(code) => write!(f, "Command exited with status {code}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::ExecutionFailedGeneric(Box::new(error))
    }
}

impl From<EnvironmentError> for Error {
    fn from(error: EnvironmentError) -> Self {
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

    #[test]
    fn test_non_zero_exit_error() {
        let error = Error::NonZeroExit(3);
        assert_eq!(format!("{}", error), "Command exited with status 3");
        assert_eq!(error.exit_code(), 3);
        assert_eq!(Error::CommandFailed("x".to_string()).exit_code(), 1);
    }

    #[test]
    fn test_command_failed_error() {
        let error = Error::CommandFailed("test_error".to_string());
        assert_eq!(format!("{}", error), "Command failed: 'test_error'");
    }
}
