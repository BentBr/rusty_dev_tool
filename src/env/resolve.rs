use crate::env::enums::compose::Compose;
use crate::env::enums::shell::Enum as CompletionShell;
use crate::error::environment::Error as EnvironmentError;
use crate::error::update::Error as UpdateError;
use os_info::{Info, Type};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

pub fn shell() -> Result<CompletionShell, EnvironmentError> {
    let shell = env::var("SHELL")?;

    match shell.as_str() {
        "/usr/bin/bash" | "/bin/bash" => Ok(CompletionShell::Bash),
        "/usr/bin/zsh" | "/bin/zsh" => Ok(CompletionShell::Zsh),
        "/usr/bin/fish" | "/bin/fish" => Ok(CompletionShell::Fish),
        "/usr/bin/pwsh" | "/bin/pwsh" => Ok(CompletionShell::PowerShell),
        "/usr/bin/elvish" | "/bin/elvish" => Ok(CompletionShell::Elvish),
        _ => Err(EnvironmentError::ShellNotSupported(shell)),
    }
}

pub fn language_framework(
    file_path: &str,
) -> Result<crate::env::enums::language::Enum, EnvironmentError> {
    let file = File::open(Path::new(file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.to_string()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.contains("MAIN_SERVICE") {
            return crate::env::enums::language::Enum::from_main_service(&line);
        }
    }

    Err(EnvironmentError::NoMainServiceDefined())
}

pub fn compose(file_path: &str) -> Result<Compose, EnvironmentError> {
    let file = File::open(Path::new(&file_path))
        .map_err(|_| EnvironmentError::ComposeFileNotReadable(file_path.to_string()))?;

    for line in io::BufReader::new(file).lines().map_while(Result::ok) {
        if line.eq("x-mutagen:") {
            return Ok(Compose::Mutagen);
        }
    }

    Ok(Compose::Docker)
}

pub fn binary_name(os: &Info) -> Result<String, UpdateError> {
    match os.os_type() {
        Type::Macos => match env::consts::ARCH {
            "x86_64" => Ok("rdt-macos-x86_64-".to_string()),
            "aarch64" => Ok("rdt-macos-aarch64-".to_string()),
            architecture => Err(UpdateError::UnsupportedArchitecture(
                architecture.to_string(),
            )),
        },
        Type::Linux
        | Type::Arch
        | Type::Ubuntu
        | Type::Debian
        | Type::CentOS
        | Type::Redhat
        | Type::FreeBSD => Ok("rdt-linux-x86_64-".to_string()),
        os => Err(UpdateError::UnsupportedOs(os.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env::temp_dir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_shell() {
        env::set_var("SHELL", "/bin/bash");
        assert_eq!(shell().unwrap(), CompletionShell::Bash);

        env::set_var("SHELL", "/bin/zsh");
        assert_eq!(shell().unwrap(), CompletionShell::Zsh);

        env::set_var("SHELL", "/usr/bin/fish");
        assert_eq!(shell().unwrap(), CompletionShell::Fish);

        env::set_var("SHELL", "/usr/bin/pwsh");
        assert_eq!(shell().unwrap(), CompletionShell::PowerShell);

        env::set_var("SHELL", "/usr/bin/elvish");
        assert_eq!(shell().unwrap(), CompletionShell::Elvish);

        env::set_var("SHELL", "/not/valid/shell");
        assert!(shell().is_err());
    }

    #[test]
    fn test_get_language_framework_enum() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("compose.yml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "some stuff\nMAIN_SERVICE=rust").unwrap();

        assert_eq!(
            language_framework(file_path.to_str().unwrap()).unwrap(),
            crate::env::enums::language::Enum::Rust
        );
    }

    #[test]
    fn test_get_language_framework_enum_fail() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("docker-compose.yml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "more stuff\nNo_service=none").unwrap();

        assert!(language_framework(file_path.to_str().unwrap()).is_err());
    }

    #[test]
    fn test_get_compose_enum() {
        let dir = temp_dir();
        let file_path = dir.as_path().join("compose.yaml");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "some stuff\nx-mutagen:").unwrap();

        assert_eq!(
            compose(file_path.to_str().unwrap()).unwrap(),
            Compose::Mutagen
        );

        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "other content").unwrap();

        assert_eq!(
            compose(file_path.to_str().unwrap()).unwrap(),
            Compose::Docker
        );
    }

    //Todo: test binary_name
}
