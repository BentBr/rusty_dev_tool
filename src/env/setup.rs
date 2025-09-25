use crate::env::config::{get_config_without_local, merge_configs, Config, PathOptions};
use crate::env::enums::compose::Compose;
use crate::env::enums::language::Enum as LanguageFramework;
use crate::env::home_config::{get_or_create, HomeConfig};
use crate::env::local_config::{get, LocalConfig};
use crate::env::resolve::compose;
use crate::env::resolve::language_framework;
use crate::error::environment::Error as EnvironmentError;
use regex::Regex;
use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};

pub fn init(restore: bool, update: bool, completions: bool) -> Result<Config, Box<dyn Error>> {
    let home_config: HomeConfig = get_or_create(restore)?;

    check_home_dir_is_current_dir()?;

    if restore || update || completions {
        return Ok(get_config_without_local(home_config));
    }

    let local_config: Option<LocalConfig> = get()?;
    let compose;
    let language_framework;

    // This is a bit nasty - but we must get the local config to know if compose will be a thing here
    // Setting those to default values
    if let Some(local_config) = &local_config {
        if local_config.no_docker_compose {
            compose = Compose::DefaultNotUsable;
            language_framework = LanguageFramework::DefaultNotUsable;
        } else {
            compose = check_docker_compose_setup()?;
            language_framework = check_language_framework_setup()?;
        }
    } else {
        compose = check_docker_compose_setup()?;
        language_framework = check_language_framework_setup()?;
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

    compose(compose_file_path.to_string_lossy().as_ref()).map_err(|_| {
        EnvironmentError::DockerComposeNotInstalled(local_dir.to_string_lossy().to_string())
    })
}

fn check_language_framework_setup() -> Result<LanguageFramework, EnvironmentError> {
    let local_dir = PathOptions::new()
        .get_local_working_dir()
        .map_err(|_| EnvironmentError::LocalConfigDirNotFound(String::from("project dir")))?;

    let compose_file_path = get_compose_file(local_dir.as_path())?;

    language_framework(compose_file_path.to_string_lossy().as_ref())
}

pub fn get_compose_file(local_dir: &Path) -> Result<PathBuf, EnvironmentError> {
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

fn check_home_dir_is_current_dir() -> Result<(), EnvironmentError> {
    let current_dir = env::current_dir()?;
    let home_dir = dirs::home_dir().ok_or(EnvironmentError::HomeDirNotFound())?; // This should never happen

    if current_dir.as_path().eq(home_dir.as_path()) {
        Err(EnvironmentError::HomeDirIsCurrentDir(
            home_dir.to_string_lossy().to_string(),
        ))
    } else {
        Ok(())
    }
}

pub fn get_string_via_regex<'a>(string: &'a str, regex: &Regex) -> Option<&'a str> {
    if let Some(captures) = regex.captures(string) {
        if let Some(service) = captures.get(1) {
            return Some(service.as_str());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use env::temp_dir;
    use std::fs;
    use std::fs::File;

    #[test]
    fn test_get_compose_file() {
        let dir = temp_dir().join("test1_sub");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.clone().as_path().join("docker-compose.yaml");
        File::create(&file_path).unwrap();

        let new_file_path = dir.clone().as_path().join("docker-compose.yaml");

        assert_eq!(get_compose_file(dir.as_path()).unwrap(), new_file_path);
    }

    #[test]
    fn test_get_string_via_regex() {
        let regex = Regex::new(r"(\d+)").unwrap();
        let string = "Hello123World";
        assert_eq!(get_string_via_regex(string, &regex), Some("123"));

        let regex = Regex::new(r"(World)").unwrap();
        assert_eq!(get_string_via_regex(string, &regex), Some("World"));

        let regex = Regex::new(r"(NotFound)").unwrap();
        assert_eq!(get_string_via_regex(string, &regex), None);
    }
}
