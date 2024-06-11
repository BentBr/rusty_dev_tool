use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UpdateError {
    FailedToGetLatestVersion(String),
    FailedToReadVersion(String),
    UpdateGeneric(Box<dyn Error>),
    UpdateCheckFailed(String),
    UpdateDownloadError(String),
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
            UpdateError::UpdateDownloadError(string) => {
                write!(f, "Failed to download update: '{}'. Maybe try again later (if release is being built)", string)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_to_get_latest_version_error() {
        let error = UpdateError::FailedToGetLatestVersion("test_uri".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to get latest version: 'test_uri'. Maybe check your internet connection?"
        );
    }

    #[test]
    fn test_failed_to_read_version_error() {
        let error = UpdateError::FailedToReadVersion("test_string".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to read version: 'test_string'."
        );
    }

    #[test]
    fn test_update_generic_error() {
        let error = UpdateError::UpdateGeneric(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test_error",
        )));
        assert_eq!(format!("{}", error), "Error during update: 'test_error'.");
    }

    #[test]
    fn test_update_check_failed_error() {
        let error = UpdateError::UpdateCheckFailed("test_string".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to check for updates: 'test_string'."
        );
    }

    #[test]
    fn test_update_download_error() {
        let error = UpdateError::UpdateDownloadError("test_string".to_string());
        assert_eq!(format!("{}", error), "Failed to download update: 'test_string'. Maybe try again later (if release is being built)");
    }
}
