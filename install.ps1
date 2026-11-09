# Ferrix CLI Installer for Windows PowerShell
$ErrorActionPreference = "Stop"

$Repo = "soumitra-das-dev/ferrix-ui"
$Version = "0.1.0"
$InstallDir = Join-Path $HOME ".ferrix\bin"
$BinaryName = "ferrix-x86_64-pc-windows-msvc.exe"
$DestPath = Join-Path $InstallDir "ferrix.exe"
$DownloadUrl = "https://github.com/$Repo/releases/download/v$Version/$BinaryName"

Write-Host "⚡ Installing Ferrix CLI v$Version for Windows..." -ForegroundColor Cyan

if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

try {
    Write-Host "⬇️ Downloading $DownloadUrl..."
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $DestPath
} catch {
    Write-Warning "Prebuilt binary release not found online."
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "🔨 Compiling locally from source using cargo..."
        cargo install ferrix-cli
    } else {
        Write-Error "Could not install Ferrix. Please install Rust (https://rustup.rs) and run: cargo install ferrix-cli"
        exit 1
    }
}

# Add to user PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "ℹ️ Added $InstallDir to user PATH."
}

Write-Host "✨ Ferrix CLI installed successfully!" -ForegroundColor Green
Write-Host "Open a new terminal and run 'ferrix --help' to begin."
