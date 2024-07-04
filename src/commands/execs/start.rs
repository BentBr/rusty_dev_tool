use crate::commands::command::run;
use crate::commands::command::Command;
use crate::env::config::Config;
use crate::env::enums::language::Enum as LanguageFramework;
use crate::error::command::Error as CommandError;
use crate::error::environment::Error::NoComposeServiceDefined;
use colored::Colorize;

pub struct Start;

impl Command for Start {
    fn execute(&self, config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        let compose_str = config.compose.to_string();
        let binding = format!(
            "{} pull && {} up -d --build && {} exec -T {}",
            compose_str,
            compose_str,
            compose_str,
            get_main_service_install(config)
        );
        let command = binding.as_str();

        println!(
            "{}",
            format!(
                "Executing start command with '{}'",
                config.language_framework
            )
            .blue()
        );

        run(command)
    }

    fn alias(&self) -> String {
        "start".to_string()
    }

    fn description(&self) -> String {
        "Generic setup start command".to_string()
    }
}

fn get_main_service_install(config: &Config) -> String {
    match config.language_framework {
        LanguageFramework::Php => "php composer install".to_string(),
        LanguageFramework::Node => "node yarn install".to_string(),
        LanguageFramework::Rust => "rust cargo build".to_string(),
        LanguageFramework::DefaultNotUsable => {
            NoComposeServiceDefined("DefaultNotUsable".to_string()).to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::config::Config;
    use crate::env::enums::language::Enum as LanguageFramework;
    use crate::error::environment::Error::NoComposeServiceDefined;

    #[test]
    fn test_get_main_service_install_php() {
        let config = Config {
            language_framework: LanguageFramework::Php,
            ..Config::default()
        };
        assert_eq!(get_main_service_install(&config), "php composer install");
    }

    #[test]
    fn test_get_main_service_install_node() {
        let config = Config {
            language_framework: LanguageFramework::Node,
            ..Config::default()
        };
        assert_eq!(get_main_service_install(&config), "node yarn install");
    }

    #[test]
    fn test_get_main_service_install_rust() {
        let config = Config {
            language_framework: LanguageFramework::Rust,
            ..Config::default()
        };
        assert_eq!(get_main_service_install(&config), "rust cargo build");
    }

    #[test]
    fn test_get_main_service_install_default_not_usable() {
        let config = Config {
            language_framework: LanguageFramework::DefaultNotUsable,
            ..Config::default()
        };
        assert_eq!(
            get_main_service_install(&config),
            NoComposeServiceDefined("DefaultNotUsable".to_string()).to_string()
        );
    }
}
