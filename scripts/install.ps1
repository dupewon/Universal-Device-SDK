# UDS CLI Windows Installer
param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\uds"
)

Write-Host "Installing UDS CLI..." -ForegroundColor Green

$target = "x86_64-pc-windows-msvc"

if ($Version -eq "latest") {
    $downloadUrl = "https://github.com/dupewon/Universal-Device-SDK/releases/latest/download/uds-$target.zip"
} else {
    $downloadUrl = "https://github.com/dupewon/Universal-Device-SDK/releases/download/v$Version/uds-$target.zip"
}

$tmpDir = "$env:TEMP\uds-install"
New-Item -ItemType Directory -Path $tmpDir -Force | Out-Null

Write-Host "Downloading UDS for Windows..." -ForegroundColor Yellow
Invoke-WebRequest -Uri $downloadUrl -OutFile "$tmpDir\uds.zip"
Expand-Archive -Path "$tmpDir\uds.zip" -DestinationPath $tmpDir -Force

New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Copy-Item "$tmpDir\uds.exe" "$InstallDir\uds.exe" -Force

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    $env:Path = "$env:Path;$InstallDir"
}

Write-Host "Installation complete!" -ForegroundColor Green
Write-Host "Run 'uds --help' to get started." -ForegroundColor Cyan
