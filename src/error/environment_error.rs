use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EnvironmentError {
    DockerComposeNotInstalled(String),
    ComposeFileNotReadable(String),
    ComposeFileNotFound(String),
    LocalConfigDirNotFound(String),
    NotExistingServiceConfig(String),
    NoMainServiceDefined(),
}

impl Error for EnvironmentError {}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvironmentError::DockerComposeNotInstalled(dir) => {
                write!(f, "Docker compose setup not found in dir '{}'", dir)
            }
            EnvironmentError::ComposeFileNotReadable(path_file) => {
                write!(f, "Compose file '{}' not readable", path_file)
            }
            EnvironmentError::ComposeFileNotFound(dir) => {
                write!(f, "Local compose file not found in location '{}'", dir)
            }
            EnvironmentError::LocalConfigDirNotFound(dir) => {
                write!(f, "Local config dir not found in location '{}'", dir)
            }
            EnvironmentError::NotExistingServiceConfig(service) => {
                write!(
                    f,
                    "The main service '{}' definition does not exist",
                    service
                )
            }
            EnvironmentError::NoMainServiceDefined() => {
                write!(f, "Non of your services in compose.yaml has the environment 'MAIN_SERVICE=node|rust|php defined")
            }
        }
    }
}
