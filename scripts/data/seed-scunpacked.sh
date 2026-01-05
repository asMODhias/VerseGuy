#!/usr/bin/env bash
set -euo pipefail

# Seed the scunpacked DB with the sample fixtures
ROOT_DIR=$(git rev-parse --show-toplevel)
cd "$ROOT_DIR"

DB_DIR="data/scunpacked/db"
mkdir -p "$DB_DIR"

cargo run --bin importer -p plugins-adapter-scunpacked -- --file plugins/adapters/scunpacked/tests/fixtures/sample_ships.json --db "$DB_DIR"

echo "Seeding finished. DB at $DB_DIR"
