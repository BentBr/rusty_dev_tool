use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    DockerComposeNotInstalled(String),
    ComposeFileNotReadable(String),
    ComposeFileNotFound(String),
    LocalConfigDirNotFound(String),
    NotExistingServiceConfig(String),
    NoMainServiceDefined(),
}

impl StdError for Error {}

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
}
