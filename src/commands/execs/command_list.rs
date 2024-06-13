use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COMMAND_LIST: HashMap<String, String> = {
        let mut list: HashMap<String, String> = HashMap::default();

        // Command's name and description
        // Those are being used for auto-registering of commands as well as auto-generating
        // completion
        list.insert("start".to_string(), "Generic setup start command".to_string());
        list.insert("stop".to_string(), "Generic setup stop command".to_string());
        list.insert("shell".to_string(), "Exec'ing into the node container and takes an additional argument as the target to shell into (container name)".to_string());
        list.insert("chown".to_string(), "Chowning project inside the main container and takes an additional argument as the group and user like root:root".to_string());
        list.insert("build".to_string(), "Building the local image again via docker buildx".to_string());

        list
    };
}
