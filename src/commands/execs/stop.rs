use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
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

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("stop")
            .expect("'stop' command not found in command list")
            .0
            .to_string()
    }
}
