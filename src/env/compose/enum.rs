use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum Compose {
    DockerCompose,
    MutagenCompose,
}

impl fmt::Display for Compose {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DockerCompose => write!(f, "docker-compose",),
            Self::MutagenCompose => write!(f, "mutagen-compose",),
        }
    }
}
