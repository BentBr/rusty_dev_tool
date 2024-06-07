use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command_error::CommandError;

pub struct Shell;

impl Command for Shell {
    fn execute(&self, config: &Config) -> Result<(), CommandError> {
        let binding = format!("{} exec {} bash", config.compose, config.language_framework).to_string();
        let command = binding.as_str();

        run_command(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("shell")
            .expect("'shell' command not found in command list")
            .0
            .to_string()
    }
}
