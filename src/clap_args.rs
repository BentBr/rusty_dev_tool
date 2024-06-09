use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use clap::{Arg, ArgAction, Command as ClapCommand};
use clap_complete::{generate, Shell};
use std::collections::HashMap;
use std::error::Error;
use std::io;

pub const CONFIG_RESTORE: &str = "config-restore";
pub const SELF_UPDATE: &str = "self-update";
pub const GENERATE_COMPLETIONS: &str = "generate-completions";
pub const OPTIONAL_ARGUMENT: &str = "optional-argument";

pub fn get_clap_matches(config: Result<Config, Box<dyn Error>>) -> ClapCommand {
    let version = env!("CARGO_PKG_VERSION");

    let app = ClapCommand::new("Rusty Dev Tool")
        .version(version)
        .author("Bent Brüggemann <mail@bent-brueggemann.de>")
        .about("Docker helper command line tool for developers with docker-compose setups.")
        .arg(
            Arg::new(GENERATE_COMPLETIONS)
                .long(GENERATE_COMPLETIONS)
                .value_name("SHELL")
                .help("Generate shell completions")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new(CONFIG_RESTORE)
                .long(CONFIG_RESTORE)
                .help("Restores the home config environment")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(SELF_UPDATE)
                .long(SELF_UPDATE)
                .help("Updates this very tool to the latest version")
                .action(ArgAction::SetTrue),
        );
    // Todo: fix the shell completion
    //.possible_values(["bash", "zsh", "fish", "powershell", "elvish"])
    //.takes_value(true))

    register_subcommands(app, config)
}

fn register_subcommands(
    mut app: ClapCommand,
    config: Result<Config, Box<dyn Error>>,
) -> ClapCommand {
    // Todo: we want to get rid of this list as well. In best case all data is coming from the command definitions itself
    let mut commands_map: HashMap<String, String> = COMMAND_LIST.clone();

    match &config {
        Ok(config) => {
            for (_, command) in config.commands.clone() {
                commands_map.insert(
                    command.alias.clone(),
                    format!("Custom command: {}", command.command.clone()),
                );
            }
        }
        Err(_) => {
            // Todo: find config error: TomlNotReadable (or similar)
            // We are ignoring errors on purpose here as the config is being parsed and checked
            // in the "correct" init function later on
        }
    }

    // Todo: This is not pretty. Is there a better solution?
    let commands_iter = Box::leak(
        commands_map
            .into_iter()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
    .iter();

    for (name, description) in commands_iter {
        app = app.subcommand(
            ClapCommand::new(name.as_str())
                .about(description.as_str())
                .arg(
                    Arg::new(OPTIONAL_ARGUMENT)
                        .help("Optional local argument")
                        .num_args(0..=1)
                        .required(false),
                ),
        );
    }

    app
}

#[allow(dead_code)]
fn generate_completions(shell: &Shell) {
    let mut cmd = ClapCommand::new("abc")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Command-line application")
        .subcommand(ClapCommand::new("start").about("Starts the application"))
        .subcommand(ClapCommand::new("stop").about("Stops the application"));
    //todo: add more...

    //todo: add global custom commands

    generate(*shell, &mut cmd, "abc", &mut io::stdout());
}

// todo: add the generation script for auto-completion
//
// fn generate_completions(shell: &Shell) {
//     let mut cmd = ClapCommand::new("abc")
//         .version("1.0")
//         .author("Your Name <your.email@example.com>")
//         .about("Command-line application");
//
//     let registry = CommandRegistry::new();
//     for (name, _command) in registry.iter() {
//         cmd = cmd.subcommand(ClapCommand::new(name.as_str())
//             .about(&format!("Runs the {} command", name)));
//     }
//
//     generate(shell.clone(), &mut cmd, "abc", &mut io::stdout());
// }
