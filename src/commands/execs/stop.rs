use crate::commands::command::run;
use crate::commands::command::Command;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use colored::Colorize;

pub struct Stop;

impl Command for Stop {
    fn execute(&self, config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        let binding = format!("{} down", config.compose);
        let command = binding.as_str();

        println!("{}", "Executing stop command".blue());

        run(command)
    }

    fn alias(&self) -> String {
        "stop".to_string()
    }

    fn description(&self) -> String {
        "Generic setup stop command".to_string()
    }
}
