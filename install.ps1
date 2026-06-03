$ErrorActionPreference = "Stop"

$Repo = "OxgeneratorLabs/oxgenerator"
$BinName = "oxgen.exe"
$InstallDir = if ($env:OXGEN_INSTALL_DIR) { $env:OXGEN_INSTALL_DIR } else { "$env:USERPROFILE\.local\bin" }

$Arch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture

switch ($Arch) {
    "X64" {
        $Target = "x86_64-pc-windows-msvc"
    }
    "Arm64" {
        $Target = "aarch64-pc-windows-msvc"
    }
    default {
        throw "unsupported architecture: $Arch"
    }
}

$ArchiveName = "oxgen-$Target.zip"
$DownloadUrl = "https://github.com/$Repo/releases/latest/download/$ArchiveName"
$TempDir = New-Item -ItemType Directory -Force -Path ([System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), [System.Guid]::NewGuid().ToString()))

Write-Host "Downloading oxgen from $DownloadUrl"

$ArchivePath = Join-Path $TempDir $ArchiveName
Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath

Expand-Archive -Path $ArchivePath -DestinationPath $TempDir -Force

$ExtractedBinary = Join-Path $TempDir $BinName

if (!(Test-Path $ExtractedBinary)) {
    throw "archive does not contain $BinName"
}

New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null

$Destination = Join-Path $InstallDir $BinName
Move-Item -Path $ExtractedBinary -Destination $Destination -Force

Write-Host "oxgen installed to $Destination"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")

if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$InstallDir;$UserPath", "User")
    Write-Host ""
    Write-Host "Added $InstallDir to your user PATH."
    Write-Host "Restart your terminal before running oxgen."
}

Write-Host ""
& $Destination --version
