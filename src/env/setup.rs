use crate::env::compose::r#enum::Compose;
use crate::env::config::{get_config_without_local, merge_configs, Config, PathOptions};
use crate::env::home_config::{get_or_create, HomeConfig};
use crate::env::language::r#enum::LanguageFramework;
use crate::env::local_config::{get, LocalConfig};
use crate::error::environment::Error as EnvironmentError;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

pub fn init(restore: bool, update: bool) -> Result<Config, Box<dyn Error>> {
    let home_config: HomeConfig = get_or_create(restore)?;

    if restore || update {
        return Ok(get_config_without_local(home_config));
    }

    let local_config: Option<LocalConfig> = get()?;
    let compose;
    let language_framework;

    // This is a bit nasty - but we must get the local config to know if compose will be a thing here
    // Setting those to default values
    match &local_config {
        Some(local_config) => {
            if local_config.no_docker_compose {
                compose = Compose::DefaultNotUsable;
                language_framework = LanguageFramework::DefaultNotUsable;
            } else {
                compose = check_docker_compose_setup()?;
                language_framework = check_language_framework_setup()?;
            }
        }
        None => {
            compose = check_docker_compose_setup()?;
            language_framework = check_language_framework_setup()?;
        }
    }

    Ok(merge_configs(
        home_config,
        local_config,
        compose,
        language_framework,
    ))
}

pub fn init_custom_commands() -> Result<Config, Box<dyn Error>> {
    let home_config: HomeConfig = get_or_create(false)?;
    let local_config = get()?;

    // We are using the default compose and language framework here as they are not needed
    // for the custom commands
    Ok(merge_configs(
        home_config,
        local_config,
        Compose::DefaultNotUsable,
        LanguageFramework::DefaultNotUsable,
    ))
}

fn check_docker_compose_setup() -> Result<Compose, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_working_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("project dir")))?;

    let compose_file_path = get_compose_file(&local_dir)?;

    get_compose_enum(compose_file_path.to_string_lossy().as_ref()).map_err(|_| {
        EnvironmentError::DockerComposeNotInstalled(local_dir.to_string_lossy().to_string())
    })
}

fn check_language_framework_setup() -> Result<LanguageFramework, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_working_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("project dir")))?;

    let compose_file_path = get_compose_file(local_dir.as_path())?;

    get_language_framework_enum(compose_file_path.to_string_lossy().as_ref())
}

fn get_compose_enum(file_path: &str) -> Result<Compose, EnvironmentError> {
    let file = File::open(Path::new(&file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.to_string()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.eq("x-mutagen:") {
            return Ok(Compose::Mutagen);
        }
    }

    Ok(Compose::Docker)
}

fn get_compose_file(local_dir: &Path) -> Result<PathBuf, EnvironmentError> {
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

fn get_language_framework_enum(file_path: &str) -> Result<LanguageFramework, EnvironmentError> {
    let file = File::open(Path::new(file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.to_string()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.contains("MAIN_SERVICE") {
            return LanguageFramework::from_main_service(&line);
        }
    }

    Err(EnvironmentError::NoMainServiceDefined())
}
