#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -gt 0 ]; then
  patterns=("$@")
else
  patterns=("plugins/**/manifest.toml")
fi

# Build and run the manifest-validator binary
cargo build -p manifest-validator --quiet
for p in "${patterns[@]}"; do
  cargo run -p manifest-validator -- "$p"
done

echo "All checked manifests validated successfully."