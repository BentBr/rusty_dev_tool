use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Enum {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl fmt::Display for Enum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Zsh => write!(f, "zsh"),
            Self::Fish => write!(f, "fish"),
            Self::PowerShell => write!(f, "powershell"),
            Self::Elvish => write!(f, "elvish"),
        }
    }
}

impl Enum {
    pub fn to_binary_string(&self) -> String {
        match self {
            Self::Bash => "bash".to_string(),
            Self::Zsh => "zsh".to_string(),
            Self::Fish => "fish".to_string(),
            Self::PowerShell => "pwsh".to_string(),
            Self::Elvish => "elvish".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Enum::Bash), "bash");
        assert_eq!(format!("{}", Enum::Zsh), "zsh");
        assert_eq!(format!("{}", Enum::Fish), "fish");
        assert_eq!(format!("{}", Enum::PowerShell), "powershell");
        assert_eq!(format!("{}", Enum::Elvish), "elvish");
    }

    #[test]
    fn test_to_binary_string() {
        assert_eq!(Enum::Bash.to_binary_string(), "bash");
        assert_eq!(Enum::Zsh.to_binary_string(), "zsh");
        assert_eq!(Enum::Fish.to_binary_string(), "fish");
        assert_eq!(Enum::PowerShell.to_binary_string(), "pwsh");
        assert_eq!(Enum::Elvish.to_binary_string(), "elvish");
    }
}
