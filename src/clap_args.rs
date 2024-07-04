use crate::commands::command::Command;
use crate::commands::execs::generate_completions::GenerateCompletions;
use crate::commands::execs::self_update::SelfUpdate;
use crate::commands::registry::Registry;
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

    let mut app = ClapCommand::new("Rusty Dev Tool")
        .version(version)
        .author("Bent Brüggemann <mail@bent-brueggemann.de>")
        .about("Docker helper command line tool for developers with docker-compose setups.");

    app = add_flag_commands(app);

    register_subcommands(app, config)
}

fn register_subcommands(
    mut app: ClapCommand,
    config: &Result<Config, Box<dyn Error>>,
) -> ClapCommand {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    match &config {
        Ok(config) => {
            // Register the official commands
            commands = Registry::new(config).into_map();
        }
        Err(err) => {
            eprintln!("Error while parsing the config: {err}");
            //Todo: error handling!
            //return Err(err);
        }
    }

    // Leak the commands to get a 'static reference
    // // Todo: This is not pretty. Is there a better solution?
    let commands: &'static [(String, String)] = Box::leak(
        commands
            .into_values()
            .map(|command| (command.alias(), command.description()))
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    );

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

fn add_flag_commands(mut app: ClapCommand) -> ClapCommand {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();

    let generate_completions = GenerateCompletions;
    commands.insert(generate_completions.alias(), Box::new(generate_completions));

    let self_update = SelfUpdate;
    commands.insert(self_update.alias(), Box::new(self_update));

    let commands_vec: Vec<(String, String)> = commands
        .into_values()
        .map(|command| (command.alias(), command.description()))
        .collect();
    let flag_commands: &'static [(String, String)] = Box::leak(commands_vec.into_boxed_slice());

    for (alias, description) in flag_commands {
        app = app.arg(
            Arg::new(alias.as_str())
                .long(alias.as_str())
                .help(description.as_str())
                .action(ArgAction::SetTrue),
        );
    }

    app.arg(
        Arg::new(CONFIG_RESTORE)
            .long(CONFIG_RESTORE)
            .help("Restores the home config environment")
            .action(ArgAction::SetTrue),
    )
}
