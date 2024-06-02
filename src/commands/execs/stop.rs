use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::error::command_error::CommandError;

pub struct Stop;

impl Command for Stop {
    fn execute(&self) -> Result<(), CommandError> {
        println!("Executing stop command");

        run_command("docker-compose down")
    }

    fn name(&self) -> String {
        "stop".to_string()
    }

    fn description(&self) -> String {
        "Docker compose stop command".to_string()
    }
}
