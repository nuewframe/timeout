# Installer script for nuewframe-timeout on Windows
# Usage: irm https://raw.githubusercontent.com/nuewframe/timeout/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

# Configuration
$Repo = "nuewframe/timeout"
$BinaryName = "timeout.exe"
$InstallDir = "$env:LOCALAPPDATA\Programs\timeout"

Write-Host "Installing timeout for Windows..." -ForegroundColor Cyan

# Get latest release tag
try {
    $LatestRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
    $LatestTag = $LatestRelease.tag_name
} catch {
    Write-Error "Error: Could not determine latest release"
    exit 1
}

Write-Host "Latest version: $LatestTag" -ForegroundColor Green

# Construct download URL
$Target = "x86_64-pc-windows-msvc"
$Archive = "timeout-${LatestTag}-${Target}.zip"
$DownloadUrl = "https://github.com/$Repo/releases/download/$LatestTag/$Archive"

# Create temporary directory
$TmpDir = New-Item -ItemType Directory -Path "$env:TEMP\timeout-install-$(Get-Random)" -Force

try {
    Write-Host "Downloading from $DownloadUrl..." -ForegroundColor Cyan
    $ArchivePath = Join-Path $TmpDir $Archive
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ArchivePath -UseBasicParsing

    # Extract
    Write-Host "Extracting..." -ForegroundColor Cyan
    Expand-Archive -Path $ArchivePath -DestinationPath $TmpDir -Force

    # Create install directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Install binary
    Write-Host "Installing to $InstallDir..." -ForegroundColor Cyan
    $SourceBinary = Join-Path $TmpDir $BinaryName
    $DestBinary = Join-Path $InstallDir $BinaryName
    Copy-Item -Path $SourceBinary -Destination $DestBinary -Force

    Write-Host ""
    Write-Host "✓ timeout installed successfully!" -ForegroundColor Green
    Write-Host ""

    # Check if install directory is in PATH
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        Write-Host "⚠️  Note: $InstallDir is not in your PATH" -ForegroundColor Yellow
        Write-Host "   Adding it now..." -ForegroundColor Cyan
        
        # Add to user PATH
        $NewPath = "$UserPath;$InstallDir"
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        
        # Update current session PATH
        $env:Path = "$env:Path;$InstallDir"
        
        Write-Host "✓ Added to PATH. You may need to restart your terminal." -ForegroundColor Green
    }

    Write-Host ""
    Write-Host "Run 'timeout --version' to verify the installation" -ForegroundColor Cyan
    Write-Host "(You may need to restart your terminal for PATH changes to take effect)" -ForegroundColor Yellow

} finally {
    # Cleanup
    Remove-Item -Path $TmpDir -Recurse -Force -ErrorAction SilentlyContinue
}
