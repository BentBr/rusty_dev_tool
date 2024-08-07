use crate::commands::command::run;
use crate::commands::command::Command;
use crate::env::config::{Config, PathOptions};
use crate::error::command::Error as CommandError;
use crate::error::environment::Error as EnvironmentError;
use colored::Colorize;
use std::fs::File;
use std::path::Path;

pub struct Build;

impl Command for Build {
    fn execute(&self, _config: &Config, _argument: Option<&String>) -> Result<(), CommandError> {
        check_for_dockerfile()?;

        let binding = "docker buildx build .".to_string();
        let command = binding.as_str();

        println!("{}", "Building local image again".blue());

        run(command)
    }

    fn alias(&self) -> String {
        "build".to_string()
    }

    fn description(&self) -> String {
        "Building the local image again via docker buildx".to_string()
    }
}

fn check_for_dockerfile() -> Result<(), EnvironmentError> {
    let local_dir = PathOptions::new().get_local_working_dir()?;
    let dockerfile_path = local_dir.join("Dockerfile");

    File::open(Path::new(&dockerfile_path)).map_err(|_| {
        EnvironmentError::DockerFileNotFound(dockerfile_path.to_string_lossy().to_string())
    })?;

    Ok(())
}
