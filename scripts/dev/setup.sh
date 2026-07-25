#!/usr/bin/env bash
set -euo pipefail

echo "Setting up UDS development environment..."

if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

rustup default stable
rustup target add x86_64-unknown-linux-gnu aarch64-apple-darwin

echo "Setup complete!"
