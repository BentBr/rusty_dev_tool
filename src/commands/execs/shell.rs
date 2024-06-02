use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::error::command_error::CommandError;

pub struct Shell;

impl Command for Shell {
    fn execute(&self) -> Result<(), CommandError> {
        //Todo:make sure to find the main container
        run_command("docker-compose exec node sh")
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("shell")
            .expect("'shell' command not found in command list")
            .0
            .to_string()
    }
}
