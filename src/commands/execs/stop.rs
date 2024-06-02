use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::error::command_error::CommandError;

pub struct Stop;

impl Command for Stop {
    fn execute(&self) -> Result<(), CommandError> {
        println!("Executing stop command");

        run_command("docker-compose down")
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("stop")
            .expect("'stop' command not found in command list")
            .0
            .to_string()
    }
}
