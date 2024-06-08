mod clap_args;
mod commands;
mod env;
mod error;

use crate::clap_args::{get_clap_matches, CONFIG_RESTORE, GENERATE_COMPLETIONS, SELF_UPDATE};
use crate::commands::command::Command;
use crate::commands::execs::self_update::SelfUpdate;
use crate::commands::registry::CommandRegistry;
use crate::env::init::init;
use clap::ArgMatches;
use colored::Colorize;
use std::process::exit;

fn main() {
    let matches: ArgMatches = get_clap_matches();
    let restore: bool = matches.get_flag(CONFIG_RESTORE);
    let update: bool = matches.get_flag(SELF_UPDATE);
    let generate_completions: bool = matches.get_flag(GENERATE_COMPLETIONS);

    let config = match init(restore, update) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{} {}", "Error initializing environment:".red(), error);
            return;
        }
    };

    if update {
        match SelfUpdate.execute(&config) {
            Ok(_) => exit(0),
            Err(err) => {
                eprintln!("{} {}", "Error updating:".red(), err);
                exit(1);
            }
        }
    }

    let registry = CommandRegistry::new();

    // If any flag is not present, require a subcommand
    if !(generate_completions || restore || update) && matches.subcommand().is_none() {
        eprintln!("{}", "A subcommand is required".red());

        exit(1);
    }

    match matches.subcommand() {
        Some((command_name, _)) => {
            match registry.get(command_name) {
                Ok(command) => {
                    // todo: Add args to command execution
                    command.execute(&config).unwrap_or_else(|err| {
                        eprintln!(
                            "{}",
                            format!("Error executing {} command: {}", command_name, err).red(),
                        );
                    });
                }
                Err(err) => {
                    eprintln!("{} {}", "Error:".red(), err);
                }
            }
        }
        _ => exit(0),
    }
}
