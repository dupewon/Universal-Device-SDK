#!/usr/bin/env bash
set -euo pipefail

REPO="dupewon/Universal-Device-SDK"
BIN_DIR="${BIN_DIR:-/usr/local/bin}"
VERSION="${1:-latest}"

echo "Installing UDS CLI..."

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux) TARGET="x86_64-unknown-linux-gnu" ;;
    Darwin)
        case "$ARCH" in
            arm64) TARGET="aarch64-apple-darwin" ;;
            x86_64) TARGET="x86_64-apple-darwin" ;;
        esac
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please install manually: cargo install uds-cli"
        exit 1
        ;;
esac

if [ "$VERSION" = "latest" ]; then
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/uds-$TARGET.tar.gz"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/uds-$TARGET.tar.gz"
fi

echo "Downloading UDS for $TARGET..."
TMP_DIR="$(mktemp -d)"
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/uds.tar.gz"
tar xzf "$TMP_DIR/uds.tar.gz" -C "$TMP_DIR"

install -m 755 "$TMP_DIR/uds" "$BIN_DIR/uds"
echo "Installed uds to $BIN_DIR/uds"

# Install shell completions
for shell in bash zsh fish; do
    COMP_DIR="${HOME}/.${shell}_completions"
    mkdir -p "$COMP_DIR"
    cp "completions/uds.$shell" "$COMP_DIR/" 2>/dev/null || true
done

echo "Installation complete!"
echo "Run 'uds --help' to get started."
