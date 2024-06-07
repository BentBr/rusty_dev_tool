use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command_error::CommandError;
use colored::Colorize;

pub struct Chown;

impl Command for Chown {
    fn execute(&self, config: &Config) -> Result<(), CommandError> {
        //Todo: Bringing in an option for the user group
        let binding = format!(
            "{} exec --user=root -T php chown -R www-data:www-data .",
            config.compose
        )
        .to_string();
        let command = binding.as_str();

        println!("{}", "Executing Chown command".blue());

        run_command(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("chown")
            .expect("'Chown' command not found in command list")
            .0
            .to_string()
    }
}
