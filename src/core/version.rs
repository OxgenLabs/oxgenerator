use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    x: u8,
    y: u8,
    z: u8,
}

impl Version {
    pub fn new(x: u8, y: u8, z: u8) -> Version {
        Version { x, y, z }
    }

    pub fn get_local_version() -> Version {
        Version::parse(env!("CARGO_PKG_VERSION"))
    }

    pub fn get_remote_version() -> Version {
        let output = match Command::new("cargo").args(["info", "oxgen"]).output() {
            Ok(output) => output,
            Err(_) => return Version::new(0, 0, 0),
        };

        if !output.status.success() {
            return Version::new(0, 0, 0);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        Version::parse_from_cargo_info_output(&stdout)
    }

    pub fn local_version_is_lower_than_remote_version(
        local_version: Version,
        remote_version: Version,
    ) -> bool {
        local_version < remote_version
    }

    fn parse(version: &str) -> Version {
        let mut parts = version.split('.');

        let x = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        let y = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        let z = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);

        Version::new(x, y, z)
    }

    fn parse_from_cargo_info_output(output: &str) -> Version {
        let version = output
            .lines()
            .find(|line| line.trim_start().starts_with("version:"))
            .and_then(|line| line.split_once(':'))
            .map(|(_, version)| version.trim())
            .unwrap_or("0.0.0");

        Version::parse(version)
    }
}
