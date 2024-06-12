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
