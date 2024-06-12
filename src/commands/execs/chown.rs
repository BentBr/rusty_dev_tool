use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use colored::Colorize;

pub struct Chown;

impl Command for Chown {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError> {
        // We are explicitly using the optional argument for the user group
        let groups: String = argument.map_or_else(
            || "www-data:www-data".to_string(),
            ToString::to_string,
        );

        let binding = format!(
            "{} exec --user=root -T {} chown -R {} .",
            config.compose, config.language_framework, groups
        );
        let command = binding.as_str();

        println!("{}", "Executing Chown command".blue());

        run(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("chown")
            .expect("'Chown' command not found in command list")
            .0
            .to_string()
    }
}
