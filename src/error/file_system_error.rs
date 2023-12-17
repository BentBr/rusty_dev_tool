use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound(String),
    AccessNotAllowed(String),
}

impl Error for FileSystemError {}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::FileNotFound(string) => write!(f, "File not found: {}", string),
            FileSystemError::AccessNotAllowed(string) => {
                write!(f, "Not allowed to access: {}", string)
            }
        }
    }
}
