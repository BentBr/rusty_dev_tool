use crate::env::compose::compose_enum::Compose;
use crate::env::config::{merge_configs, Config, PathOptions, get_config_without_local};
use crate::env::home_config::{get_or_create_home_config, HomeConfig};
use crate::env::language::language_framework_enum::LanguageFramework;
use crate::env::local_config::{get_local_config, LocalConfig};
use crate::error::environment_error::EnvironmentError;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn init(restore: bool, update: bool) -> Result<Config, Box<dyn Error>> {
    let home_config: HomeConfig = get_or_create_home_config(restore)?;

    if restore || update {
        return Ok(get_config_without_local(home_config));
    }

    let compose = check_docker_compose_setup()?;
    let language_framework = check_language_framework_setup()?;

    let project_config: Option<LocalConfig> = get_local_config()?;
    let config: Config = merge_configs(home_config, project_config, compose, language_framework);

    Ok(config)
}

fn check_docker_compose_setup() -> Result<Compose, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("project dir")))?;

    let compose_file_path = get_compose_file(local_dir.clone())
        .map_err(|_| EnvironmentError::ComposeFileNotFound(String::from("project dir")))?;

    return get_compose_enum(compose_file_path.to_string_lossy().to_string()).map_err(|_| {
        EnvironmentError::DockerComposeNotInstalled(local_dir.to_string_lossy().to_string())
    });
}

fn check_language_framework_setup() -> Result<LanguageFramework, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("project dir")))?;

    let compose_file_path = get_compose_file(local_dir)
        .map_err(|_| EnvironmentError::ComposeFileNotFound(String::from("project dir")))?;

    return get_language_framework_enum(compose_file_path.to_string_lossy().to_string());
}

fn get_compose_enum(file_path: String) -> Result<Compose, EnvironmentError> {
    let file = File::open(Path::new(&file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.clone()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.eq("x-mutagen:") {
            return Ok(Compose::MutagenCompose);
        }
    }

    Ok(Compose::DockerCompose)
}

fn get_compose_file(local_dir: PathBuf) -> Result<PathBuf, EnvironmentError> {
    let file_names = vec![
        "compose.yaml",
        "compose.yml",
        "docker-compose.yaml",
        "docker-compose.yml",
    ];

    for file_name in file_names {
        let file_path = local_dir.join(file_name);
        if file_path.exists() {
            return Ok(file_path);
        }
    }

    Err(EnvironmentError::ComposeFileNotFound(
        local_dir.to_string_lossy().to_string(),
    ))
}

fn get_language_framework_enum(file_path: String) -> Result<LanguageFramework, EnvironmentError> {
    let file = File::open(Path::new(&file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.clone()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.contains("MAIN_SERVICE") {
            return LanguageFramework::from_main_service(&line);
        }
    }

    Err(EnvironmentError::NoMainServiceDefined())
}
