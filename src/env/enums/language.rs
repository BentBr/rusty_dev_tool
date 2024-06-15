use crate::env::setup::get_string_via_regex;
use crate::error::environment::Error as EnvironmentError;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Enum {
    Rust,
    Php,
    Node,
    DefaultNotUsable,
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust => write!(f, "rust",),
            Self::Php => write!(f, "php",),
            Self::Node => write!(f, "node",),
            Self::DefaultNotUsable => write!(f, "DefaultNotUsable"),
        }
    }
}

impl Enum {
    pub fn from_main_service(service: &str) -> Result<Self, EnvironmentError> {
        let service_string = get_main_service_via_regex(service);

        match service_string {
            Some("rust") => Ok(Self::Rust),
            Some("php") => Ok(Self::Php),
            Some("node") => Ok(Self::Node),
            _ => Err(EnvironmentError::NotExistingServiceConfig(
                service.to_string(),
            )),
        }
    }
}

fn get_main_service_via_regex(string: &str) -> Option<&str> {
    let regex = Regex::new(r"MAIN_SERVICE=(\w+)").unwrap();

    get_string_via_regex(string, &regex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Enum::Rust), "rust");
        assert_eq!(format!("{}", Enum::Php), "php");
        assert_eq!(format!("{}", Enum::Node), "node");
        assert_eq!(format!("{}", Enum::DefaultNotUsable), "DefaultNotUsable");
    }

    #[test]
    fn test_from_main_service() {
        assert_eq!(
            Enum::from_main_service("MAIN_SERVICE=rust").unwrap(),
            Enum::Rust
        );
        assert_eq!(
            Enum::from_main_service("MAIN_SERVICE=php").unwrap(),
            Enum::Php
        );
        assert_eq!(
            Enum::from_main_service("MAIN_SERVICE=node").unwrap(),
            Enum::Node
        );
        assert!(Enum::from_main_service("MAIN_SERVICE=unknown").is_err());
    }

    #[test]
    fn test_get_main_service_via_regex() {
        assert_eq!(
            get_main_service_via_regex("and some more MAIN_SERVICE=rust in that string"),
            Some("rust")
        );
        assert_eq!(get_main_service_via_regex("MAIN_SERVICE=php"), Some("php"));
        assert!(get_main_service_via_regex("none").is_none());
    }
}
