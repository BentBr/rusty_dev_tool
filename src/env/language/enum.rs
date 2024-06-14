use crate::error::environment::Error as EnvironmentError;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub enum LanguageFramework {
    Rust,
    Php,
    Node,
    DefaultNotUsable,
}

impl fmt::Display for LanguageFramework {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust => write!(f, "rust",),
            Self::Php => write!(f, "php",),
            Self::Node => write!(f, "node",),
            Self::DefaultNotUsable => write!(f, "DefaultNotUsable"),
        }
    }
}

impl LanguageFramework {
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
    let re = Regex::new(r"MAIN_SERVICE=(\w+)").unwrap();

    if let Some(captures) = re.captures(string) {
        if let Some(service) = captures.get(1) {
            return Some(service.as_str());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", LanguageFramework::Rust), "rust");
        assert_eq!(format!("{}", LanguageFramework::Php), "php");
        assert_eq!(format!("{}", LanguageFramework::Node), "node");
        assert_eq!(format!("{}", LanguageFramework::DefaultNotUsable), "DefaultNotUsable");
    }

    #[test]
    fn test_from_main_service() {
        assert_eq!(LanguageFramework::from_main_service("MAIN_SERVICE=rust").unwrap(), LanguageFramework::Rust);
        assert_eq!(LanguageFramework::from_main_service("MAIN_SERVICE=php").unwrap(), LanguageFramework::Php);
        assert_eq!(LanguageFramework::from_main_service("MAIN_SERVICE=node").unwrap(), LanguageFramework::Node);
        assert!(LanguageFramework::from_main_service("MAIN_SERVICE=unknown").is_err());
    }

    #[test]
    fn test_get_main_service_via_regex() {
        assert_eq!(get_main_service_via_regex("and some more MAIN_SERVICE=rust in that string"), Some("rust"));
        assert_eq!(get_main_service_via_regex("MAIN_SERVICE=php"), Some("php"));
        assert!(get_main_service_via_regex("none").is_none());
    }
}