# UDS Development Environment Setup (Windows)
Write-Host "Setting up UDS development environment..." -ForegroundColor Green

# Check Rust
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    winget install Rustlang.Rustup
    rustup default stable
}

# Install targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-apple-darwin

Write-Host "Setup complete!" -ForegroundColor Green
