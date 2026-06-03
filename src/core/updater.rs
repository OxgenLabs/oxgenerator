use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::core::error::OxgenError;
use crate::core::result::OxgenResult;
use crate::core::version::Version;

const GITHUB_RELEASE_BASE_URL: &str =
    "https://github.com/OxgeneratorLabs/oxgenerator/releases/latest/download";

pub fn update() -> OxgenResult<()> {
    let current_exe = env::current_exe().map_err(|error| OxgenError::Io(error.to_string()))?;

    let local_version = Version::get_local_version();

    if is_probably_installed_with_cargo(&current_exe) {
        let remote_version = Version::get_remote_crates_io_version();

        if !Version::local_version_is_lower_than_remote_version(local_version, remote_version) {
            println!("Already up to date.");
            return Ok(());
        }

        update_with_cargo()
    } else {
        let remote_version = Version::get_remote_github_release_version();

        if !Version::local_version_is_lower_than_remote_version(local_version, remote_version) {
            println!("Already up to date.");
            return Ok(());
        }

        update_from_github_release(&current_exe)
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

    fs::write(&script_path, script_content).map_err(|error| OxgenError::Io(error.to_string()))?;

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
fn update_from_github_release(current_exe: &Path) -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let install_dir = current_exe
        .parent()
        .ok_or_else(|| OxgenError::Io("failed to detect oxgen install directory".to_string()))?;

    let target = detect_unix_target()?;
    let archive_name = format!("oxgen-{target}.tar.gz");
    let archive_url = format!("{GITHUB_RELEASE_BASE_URL}/{archive_name}");
    let checksum_url = format!("{archive_url}.sha256");

    let temp_dir = env::temp_dir().join(format!("oxgen-update-{}", std::process::id()));

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir).map_err(|error| OxgenError::Io(error.to_string()))?;
    }

    fs::create_dir_all(&temp_dir).map_err(|error| OxgenError::Io(error.to_string()))?;

    let archive_path = temp_dir.join(&archive_name);
    let checksum_path = temp_dir.join(format!("{archive_name}.sha256"));

    download_file(&archive_url, &archive_path)?;
    download_file(&checksum_url, &checksum_path)?;
    verify_sha256(&archive_path, &checksum_path)?;

    let status = Command::new("tar")
        .args([
            "-xzf",
            archive_path
                .to_str()
                .ok_or_else(|| OxgenError::Io("invalid archive path".to_string()))?,
            "-C",
            temp_dir
                .to_str()
                .ok_or_else(|| OxgenError::Io("invalid temporary directory path".to_string()))?,
        ])
        .status()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !status.success() {
        return Err(OxgenError::Io(
            "failed to extract release archive".to_string(),
        ));
    }

    let extracted_binary = temp_dir.join("oxgen");

    if !extracted_binary.is_file() {
        return Err(OxgenError::Io(
            "release archive does not contain oxgen binary".to_string(),
        ));
    }

    let temporary_destination = install_dir.join("oxgen.new");
    let destination = install_dir.join("oxgen");

    fs::copy(&extracted_binary, &temporary_destination)
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    let status = Command::new("chmod")
        .args([
            "+x",
            temporary_destination
                .to_str()
                .ok_or_else(|| OxgenError::Io("invalid temporary destination path".to_string()))?,
        ])
        .status()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !status.success() {
        return Err(OxgenError::Io(
            "failed to make oxgen executable".to_string(),
        ));
    }

    fs::rename(&temporary_destination, &destination)
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    fs::remove_dir_all(&temp_dir).ok();

    println!("oxgen has been updated successfully.");
    Ok(())
}

#[cfg(windows)]
fn update_from_github_release(current_exe: &Path) -> OxgenResult<()> {
    println!("Updating oxgen from GitHub Releases...");

    let current_process_id = std::process::id();

    let install_dir = current_exe
        .parent()
        .ok_or_else(|| OxgenError::Io("failed to detect oxgen install directory".to_string()))?
        .to_string_lossy()
        .to_string();

    let script_content = format!(
        r#"$ErrorActionPreference = "Stop"

$installDir = {install_dir}
$releaseBaseUrl = "{release_base_url}"
$target = "x86_64-pc-windows-msvc"
$archiveName = "oxgen-$target.zip"
$archiveUrl = "$releaseBaseUrl/$archiveName"
$checksumUrl = "$archiveUrl.sha256"
$tempDir = Join-Path ([System.IO.Path]::GetTempPath()) "oxgen-update-{current_process_id}"

Write-Host "Waiting for oxgen to exit..."
Wait-Process -Id {current_process_id} -ErrorAction SilentlyContinue

if (Test-Path $tempDir) {{
    Remove-Item -Recurse -Force $tempDir
}}

New-Item -ItemType Directory -Force -Path $tempDir | Out-Null

$archivePath = Join-Path $tempDir $archiveName
$checksumPath = Join-Path $tempDir "$archiveName.sha256"

Write-Host "Downloading $archiveUrl"
Invoke-WebRequest -Uri $archiveUrl -OutFile $archivePath

Write-Host "Downloading $checksumUrl"
Invoke-WebRequest -Uri $checksumUrl -OutFile $checksumPath

$expectedLine = Get-Content $checksumPath | Select-Object -First 1
$expectedHash = ($expectedLine -split '\s+')[0].ToLower()
$actualHash = (Get-FileHash $archivePath -Algorithm SHA256).Hash.ToLower()

if ($expectedHash -ne $actualHash) {{
    Write-Host "checksum verification failed"
    Write-Host "expected: $expectedHash"
    Write-Host "actual:   $actualHash"
    exit 1
}}

Expand-Archive -Path $archivePath -DestinationPath $tempDir -Force

$extractedBinary = Join-Path $tempDir "oxgen.exe"
$destination = Join-Path $installDir "oxgen.exe"

if (!(Test-Path $extractedBinary)) {{
    Write-Host "release archive does not contain oxgen.exe"
    exit 1
}}

Move-Item -Path $extractedBinary -Destination $destination -Force

Remove-Item -Recurse -Force $tempDir

Write-Host "oxgen has been updated successfully."
"#,
        install_dir = powershell_single_quote(&install_dir),
        release_base_url = GITHUB_RELEASE_BASE_URL,
        current_process_id = current_process_id
    );

    let script_path = env::temp_dir().join("oxgen-release-update.ps1");

    fs::write(&script_path, script_content).map_err(|error| OxgenError::Io(error.to_string()))?;

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
fn detect_unix_target() -> OxgenResult<String> {
    let os = Command::new("uname")
        .arg("-s")
        .output()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    let arch = Command::new("uname")
        .arg("-m")
        .output()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !os.status.success() || !arch.status.success() {
        return Err(OxgenError::Io(
            "failed to detect operating system or architecture".to_string(),
        ));
    }

    let os = String::from_utf8_lossy(&os.stdout).trim().to_string();
    let arch = String::from_utf8_lossy(&arch.stdout).trim().to_string();

    let os_target = match os.as_str() {
        "Linux" => "unknown-linux-gnu",
        "Darwin" => "apple-darwin",
        _ => {
            return Err(OxgenError::Io(format!(
                "unsupported operating system: {os}"
            )));
        }
    };

    let arch_target = match arch.as_str() {
        "x86_64" | "amd64" => "x86_64",
        "arm64" | "aarch64" => "aarch64",
        _ => {
            return Err(OxgenError::Io(format!("unsupported architecture: {arch}")));
        }
    };

    Ok(format!("{arch_target}-{os_target}"))
}

#[cfg(unix)]
fn download_file(url: &str, destination: &Path) -> OxgenResult<()> {
    if command_exists("curl") {
        let status = Command::new("curl")
            .args([
                "-fsSL",
                url,
                "-o",
                destination
                    .to_str()
                    .ok_or_else(|| OxgenError::Io("invalid destination path".to_string()))?,
            ])
            .status()
            .map_err(|error| OxgenError::Io(error.to_string()))?;

        if status.success() {
            return Ok(());
        }
    }

    if command_exists("wget") {
        let status = Command::new("wget")
            .args([
                "-q",
                url,
                "-O",
                destination
                    .to_str()
                    .ok_or_else(|| OxgenError::Io("invalid destination path".to_string()))?,
            ])
            .status()
            .map_err(|error| OxgenError::Io(error.to_string()))?;

        if status.success() {
            return Ok(());
        }
    }

    Err(OxgenError::Io(
        "curl or wget is required to update oxgen".to_string(),
    ))
}

#[cfg(unix)]
fn verify_sha256(archive_path: &Path, checksum_path: &Path) -> OxgenResult<()> {
    let checksum_content =
        fs::read_to_string(checksum_path).map_err(|error| OxgenError::Io(error.to_string()))?;

    let expected_hash = checksum_content
        .split_whitespace()
        .next()
        .ok_or_else(|| OxgenError::Io("invalid checksum file".to_string()))?;

    let output = Command::new("sha256sum")
        .arg(archive_path)
        .output()
        .map_err(|error| OxgenError::Io(error.to_string()))?;

    if !output.status.success() {
        return Err(OxgenError::Io(
            "failed to compute archive checksum".to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let actual_hash = stdout
        .split_whitespace()
        .next()
        .ok_or_else(|| OxgenError::Io("invalid sha256sum output".to_string()))?;

    if expected_hash != actual_hash {
        return Err(OxgenError::Io(format!(
            "checksum verification failed: expected {}, got {}",
            expected_hash, actual_hash
        )));
    }

    Ok(())
}

#[cfg(unix)]
fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .args(["-c", &format!("command -v {command} >/dev/null 2>&1")])
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(windows)]
fn powershell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}
