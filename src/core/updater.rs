use std::env;
use std::path::Path;
use std::process::Command;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;

#[cfg(unix)]
const UNIX_INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/OxgeneratorLabs/oxgenerator/main/install.sh";

#[cfg(windows)]
const WINDOWS_INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/OxgeneratorLabs/oxgenerator/main/install.ps1";

pub fn update() -> OxgenResult<()> {
    if is_probably_installed_with_cargo() {
        update_with_cargo()
    } else {
        update_with_release_script()
    }
}

fn is_probably_installed_with_cargo() -> bool {
    let current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(_) => return false,
    };

    path_contains_cargo_bin(&current_exe)
}

fn path_contains_cargo_bin(path: &Path) -> bool {
    let path = path.to_string_lossy();

    path.contains(".cargo/bin") || path.contains(".cargo\\bin")
}

fn update_with_cargo() -> OxgenResult<()> {
    println!("Updating oxgen with cargo...");

    let status = Command::new("cargo")
        .args(["install", "oxgen", "--locked", "--force"])
        .status()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !status.success() {
        return Err(OxgenError::Io(
            "cargo install oxgen --locked --force failed".to_string(),
        ));
    }

    println!("oxgen has been updated successfully.");
    Ok(())
}

#[cfg(unix)]
fn update_with_release_script() -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let command = format!("curl -fsSL {} | sh", UNIX_INSTALL_SCRIPT_URL);

    let status = Command::new("sh")
        .args(["-c", &command])
        .status()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !status.success() {
        return Err(OxgenError::Io("release update script failed".to_string()));
    }

    Ok(())
}

#[cfg(windows)]
fn update_with_release_script() -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let command = format!("irm {} | iex", WINDOWS_INSTALL_SCRIPT_URL);

    let status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            &command,
        ])
        .status()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !status.success() {
        return Err(OxgenError::Io("release update script failed".to_string()));
    }

    Ok(())
}
