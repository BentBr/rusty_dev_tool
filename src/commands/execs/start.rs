use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::error::command_error::CommandError;

pub struct Start;

impl Command for Start {
    fn execute(&self) -> Result<(), CommandError> {
        let command = "docker-compose pull && docker-compose up -d --build";
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
