use crate::commands::command::Command;
use crate::commands::execs::chown::Chown;
use crate::commands::execs::shell::Shell;
use crate::commands::execs::start::Start;
use crate::commands::execs::stop::Stop;
use crate::error::command_error::CommandError;
use std::collections::HashMap;

/**
 * CommandRegistry
 *
 * A registry for all commands. Commands are the
 */
pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(),
        };

        registry.register(Box::new(Start));
        registry.register(Box::new(Stop));
        registry.register(Box::new(Shell));
        registry.register(Box::new(Chown));

        // Todo: register config commands (and override default commands)
        registry
    }

    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.insert(command.name(), command);
    }

    pub fn get(&self, command_name: &str) -> Result<&dyn Command, CommandError> {
        self.commands
            .get(command_name)
            .map(|command| &**command)
            .ok_or(CommandError::CommandNotFound(command_name.to_string()))
    }
}
