use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EnvironmentError {
    DockerComposeNotInstalled(String),
    LocalConfigDirNotFound(String),
}

impl Error for EnvironmentError {}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvironmentError::DockerComposeNotInstalled(dir) => {
                write!(f, "Docker compose setup not found in dir '{}'", dir)
            }
            EnvironmentError::LocalConfigDirNotFound(dir) => {
                write!(f, "Local config dir not found in location '{}'", dir)
            }
        }
    }
}
