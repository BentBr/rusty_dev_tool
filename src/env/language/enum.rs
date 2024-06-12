use crate::error::environment::Error as EnvironmentError;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub enum LanguageFramework {
    Rust,
    Php,
    Node,
}

impl fmt::Display for LanguageFramework {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust => write!(f, "rust",),
            Self::Php => write!(f, "php",),
            Self::Node => write!(f, "node",),
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
