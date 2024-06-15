use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use clap::{Arg, ArgAction, Command as ClapCommand};
use std::collections::HashMap;
use std::error::Error;

pub const CONFIG_RESTORE: &str = "config-restore";
pub const SELF_UPDATE: &str = "self-update";
pub const GENERATE_COMPLETIONS: &str = "generate-completions";
pub const OPTIONAL_ARGUMENT: &str = "optional-argument";

pub fn get_clap(config: &Result<Config, Box<dyn Error>>) -> ClapCommand {
    let version = env!("CARGO_PKG_VERSION");

    let app = ClapCommand::new("Rusty Dev Tool")
        .version(version)
        .author("Bent Brüggemann <mail@bent-brueggemann.de>")
        .about("Docker helper command line tool for developers with docker-compose setups.")
        .arg(
            Arg::new(GENERATE_COMPLETIONS)
                .long(GENERATE_COMPLETIONS)
                .help("Generate shell completions")
                .action(ArgAction::SetTrue),
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

    register_subcommands(app, config)
}

fn register_subcommands(
    mut app: ClapCommand,
    config: &Result<Config, Box<dyn Error>>,
) -> ClapCommand {
    // Todo: we want to get rid of this list as well. In best case all data is coming from the command definitions itself
    let official_commands_map: HashMap<String, String> = COMMAND_LIST.clone();
    let mut custom_commands_map: HashMap<String, String> = HashMap::new();

    match &config {
        Ok(config) => {
            for (_, command) in config.commands.clone() {
                custom_commands_map.insert(
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

    // Sort the official and custom commands
    let mut official_commands: Vec<_> = official_commands_map.into_iter().collect();
    official_commands.sort_by(|a, b| a.0.cmp(&b.0));

    let mut custom_commands: Vec<_> = custom_commands_map.into_iter().collect();
    custom_commands.sort_by(|a, b| a.0.cmp(&b.0));

    // Merge the official and custom commands
    let commands = official_commands.into_iter().chain(custom_commands);

    // Leak the commands to get a 'static reference
    // // Todo: This is not pretty. Is there a better solution?
    let commands: &'static [(String, String)] =
        Box::leak(commands.collect::<Vec<_>>().into_boxed_slice());

    for (name, description) in commands {
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
