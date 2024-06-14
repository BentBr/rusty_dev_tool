use std::path::Path;
use colored::Colorize;
use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;

pub struct Db;

impl Command for Db {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError> {
        // We are explicitly using the optional argument for the file path and name
        let path_name: String =
            argument.map_or_else(|| "dump.sql".to_string(), ToString::to_string);

        if !Path::new(&path_name.as_str()).exists() {
            return Err(CommandError::CommandFailed(format!("File not found: {path_name}")));
        }

        let binding = format!(
            "{} exec --user=root -T db mysql -u root < {}",
            config.compose, path_name,
        );
        let command = binding.as_str();

        println!("{}", "Executing Db command".blue());

        run(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("db")
            .expect("'db' command not found in command list")
            .0
            .to_string()
    }
}
