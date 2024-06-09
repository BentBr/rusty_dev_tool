use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command_error::CommandError;

pub struct Shell;

impl Command for Shell {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError> {
        // We are explicitly using the optional argument for the target to shell into
        let target: String = match argument {
            Some(target) => target.to_string(),
            None => config.language_framework.to_string(),
        };

        let binding = format!("{} exec {} bash", config.compose, target).to_string();
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
