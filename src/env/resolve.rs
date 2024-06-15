use crate::env::enums::shell::Enum as CompletionShell;
use crate::error::environment::Error as EnvironmentError;
use std::env;

pub fn shell() -> Result<CompletionShell, EnvironmentError> {
    let shell = env::var("SHELL")?;

    match shell.as_str() {
        "/bin/bash" => Ok(CompletionShell::Bash),
        "/bin/zsh" => Ok(CompletionShell::Zsh),
        "/usr/bin/fish" => Ok(CompletionShell::Fish),
        "/usr/bin/pwsh" => Ok(CompletionShell::PowerShell),
        "/usr/bin/elvish" => Ok(CompletionShell::Elvish),
        _ => Err(EnvironmentError::ShellNotSupported(shell)),
    }
}

// Todo: refactor the resolving for compose and the language framework as well
// pub fn compose() -> Result<String, crate::error::environment::Error> {
//     let compose = env::var("COMPOSE_FILE")?;
//
//     Ok(compose)
// }
//
// pub fn language_framework() -> Result<String, crate::error::environment::Error> {
//     let service = env::var("MAIN_SERVICE")?;
//
//     Ok(service)
// }

// Getting if we are on wind or unix
// pub fn os() -> Result<String, crate::error::environment::Error> {
//     let os = env::var("OS")?;
// }
