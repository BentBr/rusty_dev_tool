use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq)]
pub enum Compose {
    Docker,
    Mutagen,
    DefaultNotUsable,
}

impl fmt::Display for Compose {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Docker => write!(f, "docker-compose",),
            Self::Mutagen => write!(f, "mutagen-compose",),
            Self::DefaultNotUsable => write!(f, "DefaultNotUsable"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Compose::Docker), "docker-compose");
        assert_eq!(format!("{}", Compose::Mutagen), "mutagen-compose");
        assert_eq!(format!("{}", Compose::DefaultNotUsable), "DefaultNotUsable");
    }
}
