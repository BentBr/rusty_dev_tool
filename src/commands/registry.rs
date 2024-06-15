use crate::commands::command::{new_from_config, Command};
use crate::commands::execs::build::Build;
use crate::commands::execs::chown::Chown;
use crate::commands::execs::db::Db;
use crate::commands::execs::shell::Shell;
use crate::commands::execs::start::Start;
use crate::commands::execs::stop::Stop;
use crate::env::config::Command as CommandConfig;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use std::collections::HashMap;

/**
 * `CommandRegistry`
 *
 * A registry for all commands. Commands are the working entity of this tool.
 */
pub struct Registry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Registry {
    pub fn new(config: &Config) -> Self {
        let mut registry = Self {
            commands: HashMap::new(),
        };

        registry.register(Box::new(Start));
        registry.register(Box::new(Stop));
        registry.register(Box::new(Shell));
        registry.register(Box::new(Chown));
        registry.register(Box::new(Build));
        registry.register(Box::new(Db));

        registry.register_custom_commands(config.commands.clone());

        registry
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name(), command);
    }

    fn register_custom_commands(&mut self, commands: HashMap<String, CommandConfig>) {
        for (_, command) in commands {
            let new_command = new_from_config(command.clone());

            // If the command is already registered, it's overriding the previous one (purposefully)!
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::config::Config;
    use std::collections::HashMap;

    #[test]
    fn test_registry() {
        let mut commands = HashMap::new();
        commands.insert(
            "custom".to_string(),
            CommandConfig {
                alias: "custom".to_string(),
                command: "echo custom".to_string(),
            },
        );

        let mut config = Config::default();
        config.commands = commands;

        let registry = Registry::new(&config);

        // Test predefined commands
        assert!(registry.get("start").is_ok());
        assert!(registry.get("stop").is_ok());
        assert!(registry.get("shell").is_ok());
        assert!(registry.get("chown").is_ok());
        assert!(registry.get("build").is_ok());
        assert!(registry.get("db").is_ok());

        // Test custom command
        assert!(registry.get("custom").is_ok());

        // Test non-existent command
        assert!(registry.get("non_existent").is_err());
    }
}
