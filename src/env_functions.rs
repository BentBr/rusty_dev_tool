use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    rdt_name: String,
    update_path: String,
    commands: HashMap<String, Command>,
}

#[derive(Deserialize, Debug)]
struct Command {
    command: String,
    alias: String,
}

pub fn read_environment() {
    // let toml_file = "~/.rusty_dev_tool/config.toml".to_string();
    // let toml_data = read_file(&toml_file);
    // let config = toml::from_str(&toml_data);
    //
    // match config {
    //     //Todo: remove and use config
    //     Ok(config) => println!("{} is the config", config),
    //     Err(error) => println!("Could not read config with error: {}", error)
    // }

    let config = Config::from_file(".rusty_dev_tool/config.toml");
    // Example of how to use the config
    for (_, command) in &config.unwrap().commands {
        println!("Command alias: {}", command.alias);
        println!("Command: {}", command.command);
    }

    //println!("{:?} is the toml data", config);
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}

pub fn create_env_file(_file_name: &str) {
    //fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name.to_string()).expect("Failed to open config.toml file");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read config.toml data");

    return data;
}
