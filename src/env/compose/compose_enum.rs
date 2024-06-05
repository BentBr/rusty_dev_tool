pub enum Compose {
    DockerCompose,
    MutagenCompose,
}

impl Compose {
    pub fn to_string(&self) -> String {
        match self {
            Compose::DockerCompose => "docker-compose".to_string(),
            Compose::MutagenCompose => "mutagen-compose".to_string(),
        }
    }
}
