#[cfg(test)]
mod tests {
    use crate::env::config::PathOptions;
    use dirs::home_dir;
    use std::fs;

    #[test]
    fn test_find_project_env() {
        unimplemented!()
    }

    #[test]
    fn test_get_or_create_home_dir() {
        let test_dir = ".test_dir";

        let dir_option = PathOptions::new();

        let home_dir = &dir_option
            .with_option(String::from(test_dir))
            .get_or_create_home_dir();

        if let Ok(dir) = home_dir.as_ref() {
            // Checking if the freshly created test dir is inside the fully dir
            assert!(dir.contains(&String::from(test_dir)));

            // Deleting it again
            fs::remove_dir(dir).expect("Could not delete test dir again");
        } else {
            panic!("Could not create test dir in home path");
        }
    }

    #[test]
    fn test_read_home_dir_and_create_empty_config() {
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
    fn test_read_home_dir_config() {
        unimplemented!()
    }

    #[test]
    fn test_read_project_dir_config() {
        unimplemented!()
    }

    #[test]
    fn test_parse_config() {
        unimplemented!()
    }
}
