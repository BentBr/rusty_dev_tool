use crate::commands::execs::command_list::COMMAND_LIST;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use clap_complete::{generate, Shell};
use std::io;

pub fn get_clap_matches() -> ArgMatches {
    let version = env!("CARGO_PKG_VERSION");

    let mut app = ClapCommand::new("Rusty Dev Tool")
        .version(version)
        .author("Bent Brüggemann <mail@bent-brueggemann.de>")
        .about("Docker helper command line tool for developers with docker-compose setups.")
        .arg(
            Arg::new("generate-completions")
                .long("generate-completions")
                .value_name("SHELL")
                .help("Generate shell completions")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("config-restore")
                .long("config-restore")
                .help("Restores the home config environment")
                .action(ArgAction::SetTrue),
        );
    // Todo: fix the shell completion
    //.possible_values(["bash", "zsh", "fish", "powershell", "elvish"])
    //.takes_value(true))

    for (name, description) in COMMAND_LIST.iter() {
        app = app.subcommand(ClapCommand::new(name.as_str()).about(description.as_str()));
    }

    app.get_matches()
}

#[allow(dead_code)]
fn generate_completions(shell: &Shell) {
    let mut cmd = ClapCommand::new("abc")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Command-line application")
        .subcommand(ClapCommand::new("start").about("Starts the application"))
        .subcommand(ClapCommand::new("stop").about("Stops the application"));

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
