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
