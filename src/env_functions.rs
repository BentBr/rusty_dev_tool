use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    aliases: String,
    remote_service: String,
    update_path: String,
    service_name: String,
    environments: Environments,
}

#[derive(Deserialize, Debug)]
struct Environments {
    local: String,
    remote: String,
}

pub fn read_env_file() {
    let toml_file = "~/.rusty_dev_tool/.env.toml".to_string();
    let toml_data = read_file(&toml_file);
    let config: Config = toml::from_str(&toml_data).unwrap();

    println!("{:#?} is the config", config);
}

pub fn create_env_file(file_name: &str) {
    //fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}

fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name.to_string()).expect("Failed to open .env.toml file");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Failed to read .env.toml data");

    return data;
}
