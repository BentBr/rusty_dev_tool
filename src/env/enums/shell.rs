use std::{env, fmt};
use std::process::exit;
use crate::error::environment::Error as EnvironmentError;

#[derive(Clone, Debug)]
pub(crate) enum CompletionShell {
    Bash,
    Zsh,
    Fish,
    Powershell,
    Elvish,
}

impl fmt::Display for CompletionShell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Zsh => write!(f, "zsh"),
            Self::Fish => write!(f, "fish"),
            Self::Powershell => write!(f, "powershell"),
            Self::Elvish => write!(f, "elvish"),
        }
    }
}

impl CompletionShell {
    pub fn to_binary_string(&self) -> String {
        match self {
            Self::Bash => "bash".to_string(),
            Self::Zsh => "zsh".to_string(),
            Self::Fish => "fish".to_string(),
            Self::Powershell => "pwsh".to_string(),
            Self::Elvish => "elvish".to_string(),
        }
    }
}
