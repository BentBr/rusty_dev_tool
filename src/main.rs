mod clap_args;
mod commands;
mod env;
mod env_functions;
mod error;

use crate::clap_args::get_clap_matches;
use crate::commands::command::Command;
use crate::commands::execs::shell::Shell;
use crate::commands::execs::start::Start;
use crate::commands::execs::stop::Stop;
use crate::commands::registry::CommandRegistry;
use crate::env_functions::read_environment;

fn main() {
    /**
    First, we are reading our environment and make sure to register needed commands and settings.
     */
    read_environment();

    /**
    Then, we are creating a new registry and execute desired command.
     */
    let registry = CommandRegistry::new();

    let matches = get_clap_matches();

    match matches.subcommand() {
        Some(("start", _sub_m)) => {
            let command = Start;
            command.execute().unwrap_or_else(|err| {
                eprintln!("Error executing start command: {}", err);
            });
        }
        Some(("stop", _sub_m)) => {
            let command = Stop;
            command.execute().unwrap_or_else(|err| {
                eprintln!("Error executing stop command: {}", err);
            });
        }

        Some(("shell", _sub_m)) => {
            let command = Shell;
            command.execute().unwrap_or_else(|err| {
                eprintln!("Error executing shell command: {}", err);
            });
        }
        _ => unreachable!("Subcommand is required"),
    }

}
