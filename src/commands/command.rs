use crate::commands::custom_command::CustomCommand;
use crate::env::config::Command as ConfigCommand;
use crate::env::config::Config;
use crate::env::resolve::shell as resolve_shell;
use crate::error::command::Error as CommandError;
use std::process::{Command as SysCommand, Stdio};

pub trait Command {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError>;
    fn name(&self) -> String;
}

fn run_command_unix_sh(cmd: &str) -> Result<(), CommandError> {
    let shell = resolve_shell_program()?;

    let _ = SysCommand::new(shell)
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

fn resolve_shell_program() -> Result<String, CommandError> {
    let shell = resolve_shell()?;

    Ok(shell.to_binary_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::config::Command as ConfigCommand;
    use crate::env::config::Config;
    use crate::error::command::Error as CommandError;
    use std::env;

    struct TestCommand;

    impl Command for TestCommand {
        fn execute(
            &self,
            _config: &Config,
            _argument: Option<&String>,
        ) -> Result<(), CommandError> {
            Ok(())
        }

        fn name(&self) -> String {
            "test".to_string()
        }
    }

    #[test]
    fn test_command_trait() {
        let test_command = TestCommand;

        assert_eq!(test_command.name(), "test");
        assert!(test_command.execute(&Config::default(), None).is_ok());
    }

    #[test]
    fn test_new_from_config() {
        let config_command = ConfigCommand {
            alias: "test".to_string(),
            command: "echo test".to_string(),
        };
        let command = new_from_config(config_command);

        assert_eq!(command.name(), "test");
    }

    #[test]
    fn test_resolve_shell_program() {
        let original_shell = env::var("SHELL").unwrap();
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(resolve_shell_program().unwrap(), "fish");
        env::set_var("SHELL", original_shell);
    }
}
