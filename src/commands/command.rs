use crate::env::config::Config;
use crate::error::command_error::CommandError;
use std::process::{Command as SysCommand, Stdio};

pub trait Command {
    fn execute(&self, config: &Config) -> Result<(), CommandError>;
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

pub fn run_command(cmd: &str) -> Result<(), CommandError> {
    // Todo: need to check OS and run the command accordingly
    run_command_unix_sh(cmd)
}
