use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
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

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("generate-completions")
            .expect("'generate-completions' command not found in command list")
            .0
            .to_string()
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
