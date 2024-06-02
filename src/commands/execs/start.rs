use crate::commands::command::run_command;
use crate::commands::command::Command;
use crate::error::command_error::CommandError;

pub struct Start;

impl Command for Start {
    fn execute(&self) -> Result<(), CommandError> {
        println!("Executing start command");

        run_command("docker-compose up -d --build")
    }

    fn name(&self) -> String {
        "start".to_string()
    }

    fn description(&self) -> String {
        "Docker compose start command".to_string()
    }
}
