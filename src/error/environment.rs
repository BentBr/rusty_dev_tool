use std::error::Error as StdError;
use std::fmt;
use crate::error;

#[derive(Debug)]
pub enum Error {
    DockerComposeNotInstalled(String),
    ComposeFileNotReadable(String),
    ComposeFileNotFound(String),
    LocalConfigDirNotFound(String),
    NotExistingServiceConfig(String),
    NoMainServiceDefined(),
    NoComposeServiceDefined(String),
    DockerFileNotFound(String),
    EnvironmentGeneric(Box<dyn StdError>),
    HomeDirIsCurrentDir(String),
    HomeDirNotFound()
}

impl StdError for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::EnvironmentGeneric(Box::new(error))
    }
}

impl From<error::file_system::Error> for Error {
    fn from(error: error::file_system::Error) -> Self {
        Self::EnvironmentGeneric(Box::new(error))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DockerComposeNotInstalled(dir) => {
                write!(f, "Docker compose setup not found in dir '{dir}'")
            }
            Self::ComposeFileNotReadable(path_file) => {
                write!(f, "Compose file '{path_file}' not readable")
            }
            Self::ComposeFileNotFound(dir) => {
                write!(f, "Local compose file not found in location '{dir}'")
            }
            Self::LocalConfigDirNotFound(dir) => {
                write!(f, "Local config dir not found in location '{dir}'")
            }
            Self::NotExistingServiceConfig(service) => {
                write!(f, "The main service '{service}' definition does not exist")
            }
            Self::NoMainServiceDefined() => {
                write!(f, "Non of your services in compose.yaml has the environment 'MAIN_SERVICE=node|rust|php defined")
            }
            Self::NoComposeServiceDefined(service) => {
                write!(f, "The service '{service}' cannot be used")
            }
            Self::DockerFileNotFound(dir) => {
                write!(f, "Dockerfile not found in location '{dir}'")
            }
            Self::EnvironmentGeneric(error) => {
                write!(f, "Error during environment setup or check: '{error}'")
            }
            Self::HomeDirIsCurrentDir(dir) => {
                write!(f, "Home directory is the current directory '{dir}'. RDT must not be used here as it confuses the configurations in .rusty_dev_tool/config.toml")
            }
            Self::HomeDirNotFound() => {
                write!(f, "Home directory not found!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_compose_not_installed_error() {
        let error = Error::DockerComposeNotInstalled("test_dir".to_string());
        assert_eq!(
            format!("{}", error),
            "Docker compose setup not found in dir 'test_dir'"
        );
    }

    #[test]
    fn test_compose_file_not_readable_error() {
        let error = Error::ComposeFileNotReadable("test_file".to_string());
        assert_eq!(
            format!("{}", error),
            "Compose file 'test_file' not readable"
        );
    }

    #[test]
    fn test_compose_file_not_found_error() {
        let error = Error::ComposeFileNotFound("test_dir".to_string());
        assert_eq!(
            format!("{}", error),
            "Local compose file not found in location 'test_dir'"
        );
    }

    #[test]
    fn test_local_config_dir_not_found_error() {
        let error = Error::LocalConfigDirNotFound("test_dir".to_string());
        assert_eq!(
            format!("{}", error),
            "Local config dir not found in location 'test_dir'"
        );
    }

    #[test]
    fn test_not_existing_service_config_error() {
        let error = Error::NotExistingServiceConfig("test_service".to_string());
        assert_eq!(
            format!("{}", error),
            "The main service 'test_service' definition does not exist"
        );
    }

    #[test]
    fn test_no_main_service_defined_error() {
        let error = Error::NoMainServiceDefined();
        assert_eq!(
            format!("{}", error),
            "Non of your services in compose.yaml has the environment 'MAIN_SERVICE=node|rust|php defined"
        );
    }

    #[test]
    fn test_no_compose_service_defined_error() {
        let error = Error::NoComposeServiceDefined("test_service".to_string());
        assert_eq!(
            format!("{}", error),
            "The service 'test_service' cannot be used"
        );
    }

    #[test]
    fn test_docker_file_not_found_error() {
        let error = Error::DockerFileNotFound("test_dir_diiiir".to_string());
        assert_eq!(
            format!("{}", error),
            "Dockerfile not found in location 'test_dir_diiiir'"
        );
    }

    #[test]
    fn test_environment_generic_error() {
        let error =
            Error::EnvironmentGeneric(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test_error")));
        assert_eq!(
            format!("{}", error),
            "Error during environment setup or check: 'test_error'"
        );
    }

    #[test]
    fn test_home_dir_is_current_dir_error() {
        let error = Error::HomeDirIsCurrentDir("%home%/test_dir".to_string());
        assert_eq!(
            format!("{}", error),
            "Home directory is the current directory '%home%/test_dir'. RDT must not be used here as it confuses the configurations in .rusty_dev_tool/config.toml"
        );
    }

    #[test]
    fn test_home_dir_not_found_error() {
        let error = Error::HomeDirNotFound();
        assert_eq!(
            format!("{}", error),
            "Home directory not found!"
        );
    }
}
