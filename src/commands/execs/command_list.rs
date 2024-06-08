use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COMMAND_LIST: HashMap<String, String> = {
        let mut list: HashMap<String, String> = HashMap::default();

        // Command's name and description
        // Those are being used for auto-registering of commands as well as auto-generating
        // completion
        list.insert("start".to_string(), "Docker compose start command".to_string());
        list.insert("stop".to_string(), "Docker compose stop command".to_string());
        list.insert("shell".to_string(), "Exec'ing into the node container".to_string());
        list.insert("chown".to_string(), "Chowning project inside the main container".to_string());
        list.insert("self-update".to_string(), "Updating your beloved little helper to latest release".to_string());

        list
    };
}
