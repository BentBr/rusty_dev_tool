use crate::env::config::{Command, PathOptions, DEFAULT_CONFIG_FILE};
use crate::env::traits::FromFile;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct LocalConfig {
    pub commands: HashMap<String, Command>,
    pub local_key: String,
    pub environments: HashMap<String, Environment>,
    pub no_docker_compose: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Environment {
    pub name: String,
    pub uri: String,
}

impl FromFile for LocalConfig {}

pub fn get() -> Result<Option<LocalConfig>, Box<dyn Error>> {
    // We explicitly allow the local config to be missing
    let Ok(local_config_dir) = PathOptions::new().get_local_dir_default_config() else {
        return Ok(None);
    };

    let config_file = local_config_dir.join(DEFAULT_CONFIG_FILE.as_str());
    if config_file.exists() {
        return LocalConfig::from_file(config_file.to_string_lossy().to_string().as_str())
            .map(Some);
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use env::temp_dir;
    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_from_file() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test1.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            local_key = "test_key"
            no_docker_compose = false
            [commands]
                [commands.test_command]
                    command = "echo Hello, World!"
                    alias = "test"
            [environments]
            "#
        )
        .unwrap();

        let config = LocalConfig::from_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(config.local_key, "test_key");
        assert!(config.commands.contains_key("test_command"));
        assert!(config
            .commands
            .get("test_command")
            .unwrap()
            .command
            .eq("echo Hello, World!"));
        assert!(config
            .commands
            .get("test_command")
            .unwrap()
            .alias
            .eq("test"));
        assert!(!config.environments.contains_key("test_env"));
    }

    #[test]
    fn test_from_file_fail_environments() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test2.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            local_key = "test_key"
            no_docker_compose = false
            [commands]
                [commands.test_command]
                    command = "echo Hello, World!"
                    alias = "test"
            "#
        )
        .unwrap();

        let config = LocalConfig::from_file(file_path.to_str().unwrap());

        assert!(config.is_err());
        assert!(config
            .unwrap_err()
            .to_string()
            .contains("missing field `environments`"));
    }

    #[test]
    fn test_from_file_fail_local_key() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test3.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(
            file,
            r#"
            loacal_key = "test_key"
            no_docker_compose = false
            [commands]
                [commands.test_command]
                    command = "echo Hello, World!"
                    alias = "test"
            [environments]
            "#
        )
        .unwrap();

        let config = LocalConfig::from_file(file_path.to_str().unwrap());

        assert!(config.is_err());
        assert!(config
            .unwrap_err()
            .to_string()
            .contains("missing field `local_key`"));
    }
}
