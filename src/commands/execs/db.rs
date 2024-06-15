use crate::commands::command::run;
use crate::commands::command::Command;
use crate::commands::execs::command_list::COMMAND_LIST;
use crate::env::config::Config;
use crate::env::setup::{get_compose_file, get_string_via_regex};
use crate::error::command::Error as CommandError;
use colored::Colorize;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, Seek, SeekFrom};
use std::path::Path;
use std::{env, io};

pub struct Db;

impl Command for Db {
    fn execute(&self, config: &Config, argument: Option<&String>) -> Result<(), CommandError> {
        // We are explicitly using the optional argument for the file path and name
        let path_name: String =
            argument.map_or_else(|| "dump.sql".to_string(), ToString::to_string);

        if !Path::new(&path_name.as_str()).exists() {
            return Err(CommandError::CommandFailed(format!(
                "File not found: {path_name}"
            )));
        }

        // Get credentials from compose file
        let compose_file_path = get_compose_file(env::current_dir()?.as_path())?;
        let file = File::open(compose_file_path.clone()).map_err(|_| {
            crate::error::environment::Error::ComposeFileNotReadable(
                compose_file_path.to_string_lossy().to_string(),
            )
        })?;

        let file_ext = check_file_type(Path::new(&path_name))?;

        let db_pass = get_mysql_pass_from_file(&file)?;
        let db = get_mysql_db_from_file(&file)?;

        let binding: String = match file_ext.as_str() {
            "gz" => {
                format!(
                    "gunzip -c {} | {} exec --user=root -T db mysql -f -u root -p{} {}",
                    path_name, config.compose, db_pass, db
                )
            }
            "sql" => {
                format!(
                    "{} exec --user=root -T db mysql -f -u root -p{} {} < {}",
                    config.compose, db_pass, db, path_name
                )
            }
            _ => {
                return Err(CommandError::CommandFailed(
                    "Unsupported file type".to_string(),
                ))
            }
        };

        let command = binding.as_str();

        println!("{}", "Executing Db command".blue());

        run(command)
    }

    fn name(&self) -> String {
        COMMAND_LIST
            .get_key_value("db")
            .expect("'db' command not found in command list")
            .0
            .to_string()
    }
}

fn get_mysql_pass_from_file(file: &File) -> Result<String, CommandError> {
    extract_string_from_file(file, "MYSQL_ROOT_PASSWORD")
}

fn get_mysql_db_from_file(file: &File) -> Result<String, CommandError> {
    extract_string_from_file(file, "MYSQL_DATABASE")
}

fn extract_string_from_file(mut file: &File, string: &str) -> Result<String, CommandError> {
    let regex = Regex::new(format!(r"{string}=(\w+)").as_str()).unwrap();
    file.seek(SeekFrom::Start(0))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.contains(string) {
            let option = get_string_via_regex(&line, &regex);

            return option.map_or_else(
                || {
                    Err(CommandError::CommandFailed(format!(
                        "{string} not found in compose file"
                    )))
                },
                |value| Ok(value.to_string()),
            );
        }
    }

    Err(CommandError::CommandFailed(format!(
        "{string} not found in compose file"
    )))
}

fn check_file_type(path: &Path) -> Result<String, CommandError> {
    let path = Path::new(path);

    match path.extension() {
        Some(ext) if ext == "gz" => Ok("gz".to_string()),
        Some(ext) if ext == "sql" => Ok("sql".to_string()),
        _ => Err(CommandError::CommandFailed(
            "Unsupported file type".to_string(),
        )),
    }
}
