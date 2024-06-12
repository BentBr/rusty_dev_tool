use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FailedToGetLatestVersion(String),
    FailedToReadVersion(String),
    UpdateGeneric(Box<dyn StdError>),
    UpdateCheckFailed(String),
    UpdateDownload(String),
}

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FailedToGetLatestVersion(uri) => write!(
                f,
                "Failed to get latest version: '{uri}'. Maybe check your internet connection?"
            ),
            Self::FailedToReadVersion(string) => {
                write!(f, "Failed to read version: '{string}'.")
            }
            Self::UpdateGeneric(error) => {
                write!(f, "Error during update: '{error}'.")
            }
            Self::UpdateCheckFailed(string) => {
                write!(f, "Failed to check for updates: '{string}'.")
            }
            Self::UpdateDownload(string) => {
                write!(f, "Failed to download update: '{string}'. Maybe try again later (if release is being built)")
            }
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::UpdateGeneric(Box::new(error))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::UpdateGeneric(Box::new(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failed_to_get_latest_version_error() {
        let error = Error::FailedToGetLatestVersion("test_uri".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to get latest version: 'test_uri'. Maybe check your internet connection?"
        );
    }

    #[test]
    fn test_failed_to_read_version_error() {
        let error = Error::FailedToReadVersion("test_string".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to read version: 'test_string'."
        );
    }

    #[test]
    fn test_update_generic_error() {
        let error = Error::UpdateGeneric(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "test_error",
        )));
        assert_eq!(format!("{}", error), "Error during update: 'test_error'.");
    }

    #[test]
    fn test_update_check_failed_error() {
        let error = Error::UpdateCheckFailed("test_string".to_string());
        assert_eq!(
            format!("{}", error),
            "Failed to check for updates: 'test_string'."
        );
    }

    #[test]
    fn test_update_download_error() {
        let error = Error::UpdateDownload("test_string".to_string());
        assert_eq!(format!("{}", error), "Failed to download update: 'test_string'. Maybe try again later (if release is being built)");
    }
}
