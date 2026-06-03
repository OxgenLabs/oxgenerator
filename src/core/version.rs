use std::process::Command;

pub struct Version {
    x: u8,
    y: u8,
    z: u8
}

impl Version {
    pub fn new(x: u8, y: u8, z: u8) -> Version {
        Version {
            x,
            y,
            z
        }
    }

    pub fn get_local_version() -> Version {
        let current_version = env!("CARGO_PKG_VERSION").to_string();
        let mut parts = current_version.split(".");
        let x = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        let y = parts.next().unwrap_or("1").parse::<u8>().unwrap_or(0);
        let z = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        Version::new(x, y , z)
    }

    pub fn get_remote_version() -> Version {
        let output = Command::new("cargo")
            .args(["info", "oxgen"])
            .output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
    
        let version_line = stdout
            .lines()
            .find(|line| line.trim_start().starts_with("version:"))
            .unwrap();
        let current_version = version_line.split_once(":").unwrap().1.trim();
        let mut parts = current_version.split(".");
        let x = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        let y = parts.next().unwrap_or("1").parse::<u8>().unwrap_or(0);
        let z = parts.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
        Version::new(x, y, z)
    }
    
    pub fn remote_version_is_greater(local_version: Version, remote_version: Version) -> bool {
        if (local_version.x >= remote_version.x)
        && (local_version.y >= remote_version.y)
        && (local_version.z >= remote_version.z) {
            false
        } else {
            true
        }
    }
}
