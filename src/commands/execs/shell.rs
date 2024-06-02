use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::error::command_error::CommandError;

pub struct Shell;

impl Command for Shell {
    fn execute(&self) -> Result<(), CommandError> {
        println!("Executing shell exec command");

        run_command("docker-compose exec node sh")
    }

    fn name(&self) -> String {
        "shell".to_string()
    }

    fn description(&self) -> String {
        "Exec'ing into the node container".to_string()
    }
}
