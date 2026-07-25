#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

echo "Publishing UDS v$VERSION..."

# Update versions in workspace
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Build release artifacts
cargo build --release

# Tag and push
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"

echo "Release v$VERSION published!"
