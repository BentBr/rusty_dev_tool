use std::error::Error;
use std::fs;

pub trait FromFile {
    fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized + serde::de::DeserializeOwned,
    {
        let content = fs::read_to_string(file_path)?;
        let config: Self = toml::from_str(&content)?;

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Write;

    #[derive(Deserialize)]
    struct TestConfig {
        key: String,
    }

    impl FromFile for TestConfig {}

    #[test]
    fn test_from_file_valid_toml() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test_trait1.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, r#"key = "value""#).unwrap();

        let config = TestConfig::from_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(config.key, "value");
    }

    #[test]
    fn test_from_file_invalid_toml() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("test_trait2.toml");
        let mut file = File::create(&file_path).unwrap();

        writeln!(file, r#"keyv = "value""#).unwrap();

        let result = TestConfig::from_file(file_path.to_str().unwrap());

        assert!(result.is_err());
    }

    #[test]
    fn test_from_file_non_existent_file() {
        let result = TestConfig::from_file("non_existent_file.toml");

        assert!(result.is_err());
    }
}
