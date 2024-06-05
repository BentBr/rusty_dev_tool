use crate::env::compose::compose_enum::Compose;
use crate::env::config::{merge_configs, Config, PathOptions};
use crate::env::home_config::{get_or_create_home_config, HomeConfig};
use crate::env::local_config::{get_local_config, LocalConfig};
use crate::error::environment_error::EnvironmentError;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn init(restore: bool) -> Result<Config, Box<dyn Error>> {
    check_docker_compose_setup()?;

    let home_config: HomeConfig = get_or_create_home_config(restore)?;
    let project_config: Option<LocalConfig> = get_local_config()?;
    let config: Config = merge_configs(home_config, project_config);

    Ok(config)
}

fn check_docker_compose_setup() -> Result<Compose, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("home dir")))?;

    let file_names = vec![
        "compose.yaml",
        "compose.yml",
        "docker-compose.yaml",
        "docker-compose.yml",
    ];

    for file_name in file_names {
        let file_path = local_dir.join(file_name);
        if file_path.exists() {
            return get_compose_enum(file_path.to_string_lossy().to_string());
        }
    }

    Err(EnvironmentError::DockerComposeNotInstalled(
        local_dir.to_string_lossy().to_string(),
    ))
}

fn get_compose_enum(file_path: String) -> Result<Compose, EnvironmentError> {
    let file = File::open(&Path::new(&file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.clone()))?;

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if line.eq("x-mutagen:") {
                return Ok(Compose::MutagenCompose);
            }
        }
    }

    Ok(Compose::MutagenCompose)
}
