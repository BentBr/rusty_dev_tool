use crate::commands::command::{new_from_config_command, Command};
use crate::commands::execs::chown::Chown;
use crate::commands::execs::shell::Shell;
use crate::commands::execs::start::Start;
use crate::commands::execs::stop::Stop;
use crate::env::config::Command as CommandConfig;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use std::collections::HashMap;

/**
 * CommandRegistry
 *
 * A registry for all commands. Commands are the working entity of this tool.
 */
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new(config: &Config) -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(),
        };

        registry.register(Box::new(Start));
        registry.register(Box::new(Stop));
        registry.register(Box::new(Shell));
        registry.register(Box::new(Chown));

        registry.register_custom_commands(config.commands.clone());

        registry
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name(), command);
    }

    fn register_custom_commands(&mut self, commands: HashMap<String, CommandConfig>) {
        for (_, command) in commands {
            let new_command = new_from_config_command(command.clone());

            // If the command is already registered, it's overriding the previous one!
            self.commands.insert(command.alias, new_command);
        }
    }

    pub fn get(&self, command_name: &str) -> Result<&dyn Command, CommandError> {
        self.commands
            .get(command_name)
            .map(|command| &**command)
            .ok_or(CommandError::CommandNotFound(command_name.to_string()))
    }
}
