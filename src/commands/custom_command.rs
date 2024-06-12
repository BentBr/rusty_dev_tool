use crate::commands::command::{run, Command};
use crate::env::config::Command as ConfigCommand;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;

pub struct CustomCommand {
    pub config_command: ConfigCommand,
}

impl Command for CustomCommand {
    fn execute(&self, _config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        // Use the command from the ConfigCommand to execute the custom command
        run(&self.config_command.command)
    }

    fn name(&self) -> String {
        self.config_command.alias.clone()
    }
}
