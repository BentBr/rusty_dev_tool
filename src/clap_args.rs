use clap::builder::TypedValueParser;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn get_clap_matches() -> ArgMatches {
    ClapCommand::new("abc")
        .version("1.0")
        .author("Bent Brüggemann <mail@bent-brueggemann.de>")
        .about("Docker helper command line tool for developers with docker-compose setups.")
        .subcommand_required(true)
        .arg(
            Arg::new("generate-completions")
                .long("generate-completions")
                .value_name("SHELL")
                .help("Generate shell completions"),
        )
        //.possible_values(["bash", "zsh", "fish", "powershell", "elvish"])
        //.takes_value(true))
        .subcommand(ClapCommand::new("start").about("Starts the application"))
        .subcommand(ClapCommand::new("stop").about("Stops the application"))
        .subcommand(ClapCommand::new("shell").about("Shells into the application"))
        .get_matches()
}

fn generate_completions(shell: &Shell) {
    let mut cmd = ClapCommand::new("abc")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Command-line application")
        .subcommand(ClapCommand::new("start").about("Starts the application"))
        .subcommand(ClapCommand::new("stop").about("Stops the application"));

    generate(shell.clone(), &mut cmd, "abc", &mut io::stdout());
}

/*

    for (name, _command) in registry.iter() {
        app = app.subcommand(ClapCommand::new(name.as_str())
            .about(&format!("Runs the {} command", name)));
    }

    let matches = app.get_matches();

    if let Some(shell) = matches.value_of("generate-completions") {
        let shell: Shell = shell.parse().expect("Invalid shell type");
        generate_completions(&shell);
        return;
    }

    match matches.subcommand() {
        Some((command_name, sub_m)) => {
            if let Some(command) = registry.get(command_name) {
                let args = sub_m.value_of("config");
                command.execute(args).unwrap_or_else(|err| {
                    eprintln!("Error executing {} command: {}", command_name, err);
                });
            } else {
                eprintln!("Unknown command: {}", command_name);
            }
        },
        _ => unreachable!("Subcommand is required"),
    }
}

fn generate_completions(shell: &Shell) {
    let mut cmd = ClapCommand::new("abc")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Command-line application");

    let registry = CommandRegistry::new();
    for (name, _command) in registry.iter() {
        cmd = cmd.subcommand(ClapCommand::new(name.as_str())
            .about(&format!("Runs the {} command", name)));
    }

    generate(shell.clone(), &mut cmd, "abc", &mut io::stdout());
}
 */
