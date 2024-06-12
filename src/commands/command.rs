use crate::commands::custom_command::CustomCommand;
use crate::env::config::Command as ConfigCommand;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use std::process::{Command as SysCommand, Stdio};

pub trait Command {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError>;
    fn name(&self) -> String;
}

fn run_command_unix_sh(cmd: &str) -> Result<(), CommandError> {
    let _ = SysCommand::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn run(cmd: &str) -> Result<(), CommandError> {
    // Todo: need to check OS and run the command accordingly
    run_command_unix_sh(cmd)
}

pub fn new_from_config(config_command: ConfigCommand) -> Box<dyn Command> {
    Box::new(CustomCommand { config_command })
}
