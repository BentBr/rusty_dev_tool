use crate::commands::command::{run_command, Command};
use crate::env::config::Command as ConfigCommand;
use crate::env::config::Config;
use crate::error::command_error::CommandError;

pub struct CustomCommand {
    pub config_command: ConfigCommand,
}

impl Command for CustomCommand {
    fn execute(&self, _config: &Config) -> Result<(), CommandError> {
        // Use the command from the ConfigCommand to execute the custom command
        run_command(&self.config_command.command)
    }

    fn name(&self) -> String {
        self.config_command.alias.clone()
    }
}