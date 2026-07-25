#!/usr/bin/env bash
set -euo pipefail

echo "=== Formatting check ==="
cargo fmt --check

echo "=== Clippy ==="
cargo clippy --workspace -- -D warnings

echo "=== Tests ==="
cargo test --workspace

echo "=== Documentation ==="
cargo doc --no-deps
