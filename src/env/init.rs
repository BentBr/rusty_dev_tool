use crate::env::config::{merge_configs, Config, PathOptions};
use crate::env::home_config::{get_or_create_home_config, HomeConfig};
use crate::env::local_config::{get_local_config, LocalConfig};
use crate::error::environment_error::EnvironmentError;
use std::error::Error;

pub fn init(restore: bool) -> Result<Config, Box<dyn Error>> {
    check_docker_compose_setup()?;

    let home_config: HomeConfig = get_or_create_home_config(restore)?;
    let project_config: Option<LocalConfig> = get_local_config()?;
    let config: Config = merge_configs(home_config, project_config);

    Ok(config)
}

fn check_docker_compose_setup() -> Result<(), EnvironmentError> {
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
            return Ok(());
        }
    }

    Err(EnvironmentError::DockerComposeNotInstalled(
        local_dir.to_string_lossy().to_string(),
    ))
}
