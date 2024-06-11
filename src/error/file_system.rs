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
            Self::FileNotFound(string) => write!(f, "File not found: {string}"),
            Self::FolderNotFound(string) => write!(f, "Folder not found: {string}"),
            Self::FileWriteFailed(string) => {
                write!(f, "Failed to write to file: {string}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_not_found_error() {
        let error = FileSystemError::FileNotFound("test_file".to_string());
        assert_eq!(format!("{}", error), "File not found: test_file");
    }

    #[test]
    fn test_folder_not_found_error() {
        let error = FileSystemError::FolderNotFound("test_folder".to_string());
        assert_eq!(format!("{}", error), "Folder not found: test_folder");
    }

    #[test]
    fn test_file_write_failed_error() {
        let error = FileSystemError::FileWriteFailed("test_file".to_string());
        assert_eq!(format!("{}", error), "Failed to write to file: test_file");
    }
}
