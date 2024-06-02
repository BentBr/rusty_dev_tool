mod clap_args;
mod commands;
mod env;
mod env_functions;
mod error;

use crate::clap_args::get_clap_matches;
use crate::commands::registry::CommandRegistry;
use crate::env_functions::read_environment;

fn main() {
    read_environment();

    let matches = get_clap_matches();
    let registry = CommandRegistry::new();

    match matches.subcommand() {
        Some((command_name, _)) => {
            match registry.get(command_name) {
                Ok(command) => {
                    // todo: Add args to command execution
                    command.execute().unwrap_or_else(|err| {
                        eprintln!("Error executing {} command: {}", command_name, err);
                    });
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                }
            }
        }
        _ => unreachable!("Subcommand is required"),
    }
}
