mod clap_args;
mod commands;
mod env;
mod error;

use crate::clap_args::get_clap_matches;
use crate::commands::registry::CommandRegistry;
use crate::env::init::init;
use clap::ArgMatches;
use colored::Colorize;
use std::process::exit;

fn main() {
    let matches: ArgMatches = get_clap_matches();
    let restore: bool = matches.get_flag("config-restore");

    let config = match init(restore) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{} {}", "Error initializing environment:".red(), error);
            return;
        }
    };

    let registry = CommandRegistry::new();

    // If any flag is not present, require a subcommand
    if !matches.get_flag("generate-completions") && !restore && matches.subcommand().is_none() {
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
