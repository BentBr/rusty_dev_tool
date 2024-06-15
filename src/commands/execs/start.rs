use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::env::enums::language::LanguageFramework;
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

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("start")
            .expect("'start' command not found in command list")
            .0
            .to_string()
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
