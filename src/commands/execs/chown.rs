use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command_error::CommandError;
use colored::Colorize;

pub struct Chown;

impl Command for Chown {
    fn execute(&self, config: &Config) -> Result<(), CommandError> {
        let binding = format!("{} exec -T --user=root php chown www-data:www-data .", config.compose).to_string();
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
