use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound(String),
    FolderNotFound(String),
    FileWriteFailed(String),
}

impl Error for FileSystemError {}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::FileNotFound(string) => write!(f, "File not found: {}", string),
            FileSystemError::FolderNotFound(string) => write!(f, "Folder not found: {}", string),
            FileSystemError::FileWriteFailed(string) => {
                write!(f, "Failed to write to file: {}", string)
            }
        }
    }
}
