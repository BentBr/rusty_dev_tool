use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command_error::CommandError;

pub struct Start;

impl Command for Start {
    fn execute(&self, config: &Config) -> Result<(), CommandError> {
        let compose_str = config.compose.to_string();
        let binding = format!(
            "{} pull && {} up -d --build && {} exec -T php composer install",
            compose_str, compose_str, compose_str
        )
        .to_string();
        let command = binding.as_str();

        // Todo: get distinguished commands between php (composer install, node (yarn /npm install etc.)

        run_command(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("start")
            .expect("'start' command not found in command list")
            .0
            .to_string()
    }
}
