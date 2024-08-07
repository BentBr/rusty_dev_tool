use crate::commands::command::Command;
use crate::env::config::Config;
use crate::env::enums::shell::Enum as CompletionShell;
use crate::env::resolve::shell as resolve_shell;
use crate::error::command::Error as CommandError;
use crate::get_clap;

pub struct GenerateCompletions;

impl Command for GenerateCompletions {
    fn execute(&self, config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        let shell = resolve_shell()?;
        let clap_shell = get_clap_shell(&shell);
        let mut app = get_clap(&Ok(config.clone()));
        let app_name = config.rdt_name.clone();

        clap_complete::generate(clap_shell, &mut app, app_name, &mut std::io::stdout());

        Ok(())
    }

    fn alias(&self) -> String {
        "generate-completions".to_string()
    }

    fn description(&self) -> String {
        "Generate shell completions for the current shell".to_string()
    }
}

const fn get_clap_shell(shell: &CompletionShell) -> clap_complete::Shell {
    match shell {
        CompletionShell::Bash => clap_complete::Shell::Bash,
        CompletionShell::Fish => clap_complete::Shell::Fish,
        CompletionShell::Zsh => clap_complete::Shell::Zsh,
        CompletionShell::PowerShell => clap_complete::Shell::PowerShell,
        CompletionShell::Elvish => clap_complete::Shell::Elvish,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::enums::shell::Enum as CompletionShell;
    use clap_complete::Shell;

    #[test]
    fn test_get_clap_shell() {
        assert_eq!(get_clap_shell(&CompletionShell::Bash), Shell::Bash);
        assert_eq!(get_clap_shell(&CompletionShell::Fish), Shell::Fish);
        assert_eq!(get_clap_shell(&CompletionShell::Zsh), Shell::Zsh);
        assert_eq!(
            get_clap_shell(&CompletionShell::PowerShell),
            Shell::PowerShell
        );
        assert_eq!(get_clap_shell(&CompletionShell::Elvish), Shell::Elvish);
    }
}
