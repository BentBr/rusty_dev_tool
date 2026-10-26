//! End-to-end tests: the binary must exit with the status of the command it ran.

use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

const LOCAL_CONFIG: &str = r#"
local_key = "test"
no_docker_compose = true
[commands]
    [commands.succeed]
        execution = "true"
        alias = "succeed"
        description = "Exits with 0"
    [commands.fail]
        execution = "exit 7"
        alias = "fail"
        description = "Exits with 7"
    [commands.killed]
        execution = "kill -9 $$"
        alias = "killed"
        description = "Is killed by SIGKILL"
[environments]
"#;

/// Isolated HOME and project dir so the user's real config is never touched.
struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new(name: &str) -> Self {
        let root =
            std::env::temp_dir().join(format!("rdt-exit-code-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("home")).unwrap();
        fs::create_dir_all(root.join("project/.rusty_dev_tool")).unwrap();
        fs::write(
            root.join("project/.rusty_dev_tool/config.toml"),
            LOCAL_CONFIG,
        )
        .unwrap();

        Self { root }
    }

    fn run(&self, subcommand: &str) -> Output {
        Command::new(env!("CARGO_BIN_EXE_rusty_dev_tool"))
            .arg(subcommand)
            .current_dir(self.root.join("project"))
            .env("HOME", self.root.join("home"))
            .env("SHELL", "/bin/bash")
            .output()
            .unwrap()
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn successful_command_exits_zero() {
    let output = Sandbox::new("succeed").run("succeed");

    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn failing_command_propagates_exit_code() {
    let output = Sandbox::new("fail").run("fail");

    assert_eq!(output.status.code(), Some(7));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Command exited with status 7"));
}

#[cfg(unix)]
#[test]
fn signal_killed_command_exits_128_plus_signal() {
    let output = Sandbox::new("killed").run("killed");

    assert_eq!(output.status.code(), Some(137));
}
