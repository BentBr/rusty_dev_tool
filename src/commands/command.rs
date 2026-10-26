use crate::commands::custom_command::CustomCommand;
use crate::env::config::Command as ConfigCommand;
use crate::env::config::Config;
use crate::env::resolve::shell as resolve_shell;
use crate::error::command::Error as CommandError;
use std::process::{Command as SysCommand, ExitStatus, Stdio};

pub trait Command {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError>;
    fn alias(&self) -> String;
    fn description(&self) -> String;
}

fn run_command_unix_sh(cmd: &str) -> Result<(), CommandError> {
    let shell = resolve_shell_program()?;

    run_with_shell(&shell, cmd)
}

fn run_with_shell(shell: &str, cmd: &str) -> Result<(), CommandError> {
    let status = SysCommand::new(shell)
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()?;

    if status.success() {
        return Ok(());
    }

    Err(CommandError::NonZeroExit(exit_code(status)))
}

/// Maps a child's exit status to a process exit code, using the shell
/// convention of 128 + signal number for signal-terminated children.
fn exit_code(status: ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return 128 + signal;
        }
    }

    status.code().unwrap_or(1)
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

        fn alias(&self) -> String {
            "test".to_string()
        }

        fn description(&self) -> String {
            "Test command description".to_string()
        }
    }

    #[test]
    fn test_command_trait() {
        let test_command = TestCommand;

        assert_eq!(test_command.alias(), "test");
        assert!(test_command.execute(&Config::default(), None).is_ok());
        assert_eq!(test_command.description(), "Test command description");
    }

    #[test]
    fn test_new_from_config() {
        let config_command = ConfigCommand {
            alias: "test".to_string(),
            execution: "echo test".to_string(),
            description: "Test command description".to_string(),
        };
        let command = new_from_config(config_command);

        assert_eq!(command.alias(), "test");
        assert_eq!(command.description(), "Test command description");
    }

    #[test]
    fn test_run_success() {
        assert!(run_with_shell("sh", "true").is_ok());
    }

    #[test]
    fn test_run_propagates_exit_code() {
        match run_with_shell("sh", "exit 3") {
            Err(CommandError::NonZeroExit(code)) => assert_eq!(code, 3),
            other => panic!("expected NonZeroExit(3), got {other:?}"),
        }
    }

    #[test]
    fn test_run_false_exits_one() {
        match run_with_shell("sh", "false") {
            Err(CommandError::NonZeroExit(code)) => assert_eq!(code, 1),
            other => panic!("expected NonZeroExit(1), got {other:?}"),
        }
    }

    #[test]
    fn test_run_propagates_max_exit_code() {
        match run_with_shell("sh", "exit 255") {
            Err(CommandError::NonZeroExit(code)) => assert_eq!(code, 255),
            other => panic!("expected NonZeroExit(255), got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn test_run_maps_signal_to_128_plus_signal() {
        // SIGKILL is 9, so the shell convention yields 137
        match run_with_shell("sh", "kill -9 $$") {
            Err(CommandError::NonZeroExit(code)) => assert_eq!(code, 137),
            other => panic!("expected NonZeroExit(137), got {other:?}"),
        }
    }

    #[test]
    fn test_run_spawn_failure_is_generic_error() {
        let result = run_with_shell("/nonexistent/shell", "true");

        assert!(matches!(
            result,
            Err(CommandError::ExecutionFailedGeneric(_))
        ));
        assert_eq!(result.unwrap_err().exit_code(), 1);
    }

    #[test]
    fn test_resolve_shell_program() {
        let original_shell = env::var("SHELL").unwrap();
        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(resolve_shell_program().unwrap(), "fish");
        env::set_var("SHELL", original_shell);
    }
}
