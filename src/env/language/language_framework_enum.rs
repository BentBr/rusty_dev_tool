use crate::error::environment::EnvironmentError;
use regex::Regex;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum LanguageFramework {
    Rust,
    Php,
    Node,
}

impl fmt::Display for LanguageFramework {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LanguageFramework::Rust => write!(f, "rust",),
            LanguageFramework::Php => write!(f, "php",),
            LanguageFramework::Node => write!(f, "node",),
        }
    }
}

impl LanguageFramework {
    pub fn from_main_service(service: &str) -> Result<LanguageFramework, EnvironmentError> {
        let service_string = get_main_service_via_regex(service);

        match service_string {
            Some("rust") => Ok(LanguageFramework::Rust),
            Some("php") => Ok(LanguageFramework::Php),
            Some("node") => Ok(LanguageFramework::Node),
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
