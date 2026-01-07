#!/usr/bin/env bash
set -euo pipefail

echo "Running cargo audit..."
cargo install --version 0.18.1 cargo-audit 2>/dev/null || true
cargo audit || true

echo "Running cargo-deny..."
cargo install --version 0.11.8 cargo-deny 2>/dev/null || true
cargo deny check || true

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings -D clippy::unwrap_used || true

echo "Security checks finished"