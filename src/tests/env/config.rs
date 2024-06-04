#[cfg(test)]
mod tests {
    use crate::env::config::{DEFAULT_CONFIG_FILE, PathOptions, read_config};
    use dirs::home_dir;
    use lazy_static::lazy_static;
    use std::fs::{File, remove_dir, remove_file};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_get_or_create_home_dir() {
        let test_dir = &TEST_FOLDER.to_string();
        let dir_option = PathOptions::new();

        let home_dir = &dir_option
            .with_option(String::from(test_dir))
            .get_or_create_home_dir_default();

        if let Ok(dir) = home_dir.as_ref() {
            // Checking if the freshly created test dir is inside the fully dir
            let path = Path::new(&dir).exists();
            assert!(path);

            // Deleting it again
            remove_dir(dir).expect("Could not delete test dir in home again");
        } else {
            panic!("Could not create test dir in home path");
        }
    }

    #[test]
    fn test_get_or_create_project_dir() {
        let dir_option = PathOptions::new();

        let current_dir = &dir_option
            .with_option(String::from(TEST_FOLDER.to_string()))
            .get_or_create_project_dir_default();

        if let Ok(dir) = current_dir {
            // Checking if the freshly created test dir is inside the fully dir
            assert!(dir.exists());

            // Deleting it again
            remove_dir(dir).expect("Could not delete test dir in project again");
        } else {
            panic!("Could not get or create test dir in project path");
        }
    }

    #[test]
    fn test_find_project_env() {
        let dir_option = PathOptions::new();

        let current_dir = &dir_option
            .with_option(String::from(TEST_FOLDER.to_string()))
            .get_or_create_project_dir_default();

        if let Ok(dir) = current_dir {
            // Getting the project env
            let mut env_file_path = PathBuf::from(dir).join(DEFAULT_CONFIG_FILE.as_str());

            // Creating the project env for later testing
            println!("test file: {:?}", env_file_path);
            println!("test folder: {:?}", dir);
            assert!(dir.exists());
            let file = File::create(&env_file_path).expect("Could not create test project env");
            assert!(read_config(env_file_path.clone()).is_ok());

            let file_exists = env_file_path.exists();
            assert!(file_exists);

            // Deleting it again
            remove_file(env_file_path).expect("Could not delete test file project env in project again");
            remove_dir(dir).expect("Could not delete test folder project env in project again");
        } else {
            panic!("Could not get or create test dir in project path");
        }
    }

    #[test]
    fn test_find_home_env() {
        unimplemented!()
    }

    #[test]
    fn test_create_empty_config() {
        unimplemented!();
        if let Some(mut home) = home_dir() {
            // Append your tool's configuration directory to the home directory
            home.push(".rusty_dev_tool");

            // Get the absolute path
            let absolute_path = home.canonicalize().expect("Failed to canonicalize path");

            // Now 'absolute_path' contains the absolute path to ~/.my_tool_config
            println!(
                "Absolute path to configuration directory: {:?}",
                absolute_path
            );
        } else {
            eprintln!("Failed to get the home directory.");
        }
    }

    #[test]
    fn test_read_config() {
        unimplemented!()
    }

    #[test]
    fn test_parse_toml_config() {
        unimplemented!()
    }

    lazy_static! {
        pub static ref TEST_FOLDER: String = String::from(".test_folder");
    }
}
