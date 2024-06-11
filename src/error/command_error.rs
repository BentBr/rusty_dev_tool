use crate::error::update_error::UpdateError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    CommandNotFound(String),
    ExecutionFailedGeneric(Box<dyn Error>),
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::CommandNotFound(dir) => write!(
                f,
                "Command not found: '{}'. Maybe create a custom one in config?",
                dir
            ),
            CommandError::ExecutionFailedGeneric(error) => {
                write!(f, "Command execution failed: '{}'", error)
            }
        }
    }
}

impl From<std::io::Error> for CommandError {
    fn from(error: std::io::Error) -> Self {
        CommandError::ExecutionFailedGeneric(Box::new(error))
    }
}

impl From<UpdateError> for CommandError {
    fn from(error: UpdateError) -> Self {
        CommandError::ExecutionFailedGeneric(Box::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_not_found_error() {
        let error = CommandError::CommandNotFound("test_command".to_string());
        assert_eq!(
            format!("{}", error),
            "Command not found: 'test_command'. Maybe create a custom one in config?"
        );
    }

    #[test]
    fn test_execution_failed_generic_error() {
        let error = CommandError::ExecutionFailedGeneric(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test_error",
        )));
        assert_eq!(format!("{}", error), "Command execution failed: 'test_error'");
    }
}
