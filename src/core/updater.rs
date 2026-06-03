use std::env;
use std::path::Path;
use std::process::Command;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;
use crate::core::version::Version;

#[cfg(unix)]
const UNIX_INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/OxgeneratorLabs/oxgenerator/main/install.sh";

#[cfg(windows)]
const WINDOWS_INSTALL_SCRIPT_URL: &str =
    "https://raw.githubusercontent.com/OxgeneratorLabs/oxgenerator/main/install.ps1";

pub fn update() -> OxgenResult<()> {
    if !Version::local_version_is_lower_than_remote_version(
        Version::get_local_version(),
        Version::get_remote_version(),
    ) {
        println!("Already up to date.");
        return Ok(());
    }

    let current_exe = env::current_exe().map_err(|error| OxgenError::Io(error.to_string()))?;

    if is_probably_installed_with_cargo(&current_exe) {
        update_with_cargo()
    } else {
        update_with_release_script(&current_exe)
    }
}

fn is_probably_installed_with_cargo(current_exe: &Path) -> bool {
    path_contains_cargo_bin(current_exe)
}

fn path_contains_cargo_bin(path: &Path) -> bool {
    let path = path.to_string_lossy();

    path.contains(".cargo/bin") || path.contains(".cargo\\bin")
}

#[cfg(unix)]
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

#[cfg(windows)]
fn update_with_cargo() -> OxgenResult<()> {
    println!("Updating oxgen with cargo...");

    let current_process_id = std::process::id();

    let script_content = format!(
        r#"$ErrorActionPreference = "Stop"

Write-Host "Waiting for oxgen to exit..."
Wait-Process -Id {current_process_id} -ErrorAction SilentlyContinue

Write-Host "Updating oxgen with cargo..."
cargo install oxgen --locked --force

if ($LASTEXITCODE -ne 0) {{
    Write-Host "oxgen update failed."
    exit $LASTEXITCODE
}}

Write-Host "oxgen has been updated successfully."
"#
    );

    let script_path = env::temp_dir().join("oxgen-cargo-update.ps1");

    std::fs::write(&script_path, script_content)
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            script_path
                .to_str()
                .ok_or_else(|| OxgenError::Io("invalid update script path".to_string()))?,
        ])
        .spawn()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    println!("The update has started in a separate PowerShell process.");
    println!("oxgen.exe cannot replace itself while running on Windows.");
    println!("The update will continue after this process exits.");

    Ok(())
}

#[cfg(unix)]
fn update_with_release_script(current_exe: &Path) -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let install_dir = current_exe
        .parent()
        .ok_or_else(|| OxgenError::Io("failed to detect oxgen install directory".to_string()))?
        .to_string_lossy()
        .to_string();

    let command = format!(
        "curl -fsSL {} | OXGEN_INSTALL_DIR={} sh",
        shell_escape(UNIX_INSTALL_SCRIPT_URL),
        shell_escape(&install_dir)
    );

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
fn update_with_release_script(current_exe: &Path) -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let current_process_id = std::process::id();

    let install_dir = current_exe
        .parent()
        .ok_or_else(|| OxgenError::Io("failed to detect oxgen install directory".to_string()))?
        .to_string_lossy()
        .to_string();

    let escaped_install_dir = powershell_single_quote(&install_dir);
    let escaped_script_url = powershell_single_quote(WINDOWS_INSTALL_SCRIPT_URL);

    let script_content = format!(
        r#"$ErrorActionPreference = "Stop"

Write-Host "Waiting for oxgen to exit..."
Wait-Process -Id {current_process_id} -ErrorAction SilentlyContinue

Write-Host "Updating oxgen from GitHub Releases..."
$env:OXGEN_INSTALL_DIR = {escaped_install_dir}

iex (irm {escaped_script_url})

if ($LASTEXITCODE -ne 0) {{
    Write-Host "oxgen update failed."
    exit $LASTEXITCODE
}}

Write-Host "oxgen has been updated successfully."
"#
    );

    let script_path = env::temp_dir().join("oxgen-release-update.ps1");

    std::fs::write(&script_path, script_content)
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    Command::new("powershell")
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            script_path
                .to_str()
                .ok_or_else(|| OxgenError::Io("invalid update script path".to_string()))?,
        ])
        .spawn()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    println!("The update has started in a separate PowerShell process.");
    println!("oxgen.exe cannot replace itself while running on Windows.");
    println!("The update will continue after this process exits.");

    Ok(())
}

#[cfg(unix)]
fn shell_escape(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(windows)]
fn powershell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}
