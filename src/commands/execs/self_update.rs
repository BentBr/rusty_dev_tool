use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::error::command::Error as CommandError;
use crate::error::update::Error as UpdateError;
use colored::Colorize;
use reqwest::blocking::{get, Client};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use semver::Version;
use serde::Deserialize;
use std::io::copy;
use std::path::PathBuf;
use std::{env, fs};

pub struct SelfUpdate;

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
}

impl Command for SelfUpdate {
    fn execute(&self, config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        let check_update_needed = check_update_needed(config)
            .map_err(|error| UpdateError::UpdateCheckFailed(error.to_string()))?;

        if let Some(version) = check_update_needed {
            println!("new version: {version}");
            fetch_update(config, &version)?;

            println!(
                "{}",
                format!("Updated to latest version: {version} 🎉").green()
            );
        } else {
            println!(
                "{}",
                format!(
                    "You are already using the latest version of rusty_dev_tool '{}' ❤️",
                    env!("CARGO_PKG_VERSION")
                )
                .green()
            );
        }

        Ok(())
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("self-update")
            .expect("'self-update' command not found in command list")
            .0
            .to_string()
    }
}

fn check_update_needed(config: &Config) -> Result<Option<String>, UpdateError> {
    let url = config.meta_path.to_string();

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("RDT"));

    let client = Client::builder().default_headers(headers).build()?;

    let response: Release = client
        .get(url)
        .send()
        .map_err(|_| {
            UpdateError::FailedToGetLatestVersion("Failed to get meta data for update".to_string())
        })?
        .json()
        .map_err(|_| {
            UpdateError::FailedToReadVersion(
                "Failed to get latest version from meta data json".to_string(),
            )
        })?;

    // As GitHub has created versions like "v0.2.2" we need to remove the "v" to parse it
    let new_version = Version::parse(&response.tag_name[1..])
        .map_err(|_| UpdateError::FailedToReadVersion(response.tag_name))?;
    let local_version = Version::parse(env!("CARGO_PKG_VERSION"))
        .map_err(|_| UpdateError::FailedToReadVersion(env!("CARGO_PKG_VERSION").to_string()))?;

    if new_version > local_version {
        return Ok(Some(new_version.to_string()));
    }

    Ok(None)
}

fn fetch_update(config: &Config, tag_name: &str) -> Result<(), UpdateError> {
    let os = os_info::get();
    let binary_name = match os.os_type() {
        os_info::Type::Macos => match env::consts::ARCH {
            "x86_64" => "rdt-macos-x86_64-",
            "aarch64" => "rdt-macos-aarch64-",
            _ => panic!("Unsupported architecture"),
        },
        os_info::Type::Linux => "rdt-linux-x86_64-",
        _ => panic!("Unsupported OS"),
    };

    let download_url = format!(
        "{}/v{}/{}v{}",
        config.download_path, tag_name, binary_name, tag_name
    );

    let mut response = get(download_url.clone())?;
    let Some(content_length) = response.content_length() else {
        return Err(UpdateError::UpdateDownload(
            "Failed to get content length".to_string(),
        ));
    };

    // Not found string or empty file are less than 100 bytes
    if response.status().is_success() && content_length > 100 {
        let mut dest = {
            let bin_path = get_current_bin_path()?;
            println!("Downloading update to {}", bin_path.to_string_lossy());

            fs::File::create(bin_path)?
        };

        copy(&mut response, &mut dest)?;
    } else {
        return Err(UpdateError::UpdateDownload(download_url));
    }

    Ok(())
}

fn get_current_bin_path() -> Result<PathBuf, std::io::Error> {
    let bin_path = env::current_exe()?;
    Ok(bin_path)
}
