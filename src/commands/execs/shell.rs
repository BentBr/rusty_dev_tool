use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;

pub struct Shell;

impl Command for Shell {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError> {
        // We are explicitly using the optional argument for the target to shell into
        let target: String = argument.map_or_else(
            || config.language_framework.to_string(),
            ToString::to_string,
        );

        let binding = format!("{} exec {} bash", config.compose, target);
        let command = binding.as_str();

        run(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("shell")
            .expect("'shell' command not found in command list")
            .0
            .to_string()
    }
}
