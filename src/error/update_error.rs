use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UpdateError {
    FailedToGetLatestVersion(String),
    FailedToReadVersion(String),
    UpdateGeneric(Box<dyn Error>),
    UpdateCheckFailed(String),
}

impl Error for UpdateError {}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpdateError::FailedToGetLatestVersion(uri) => write!(
                f,
                "Failed to get latest version: '{}'. Maybe check your internet connection?",
                uri
            ),
            UpdateError::FailedToReadVersion(string) => {
                write!(f, "Failed to read version: '{}'.", string)
            }
            UpdateError::UpdateGeneric(error) => {
                write!(f, "Error during update: '{}'.", error)
            }
            UpdateError::UpdateCheckFailed(string) => {
                write!(f, "Failed to check for updates: '{}'.", string)
            }
        }
    }
}

impl From<std::io::Error> for UpdateError {
    fn from(error: std::io::Error) -> Self {
        UpdateError::UpdateGeneric(Box::new(error))
    }
}

impl From<reqwest::Error> for UpdateError {
    fn from(error: reqwest::Error) -> Self {
        UpdateError::UpdateGeneric(Box::new(error))
    }
}
