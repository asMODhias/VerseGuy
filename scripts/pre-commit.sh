#!/usr/bin/env bash
set -euo pipefail
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
echo "Running pre-commit checks (bash)..."

# 1) Formatting
cargo fmt --all -- --check

# 2) Lint (clippy) - may be slow, fail on warnings
cargo clippy --all-targets -- -D warnings

# 3) Ensure tests compile (no run) for a fast safety check
cargo test --no-run --workspace

# 4) If C++ build exists, list tests (non-failing)
if [ -d "core/build" ]; then
  if command -v ctest >/dev/null 2>&1; then
    echo "Listing C++ tests with ctest -N"
    ctest -N || true
  fi
fi

# 5) Validate plugin manifests (optional: SKIP_MANIFEST_VALIDATION=1 to skip locally)
echo "-> validate manifests"
if [ -z "${SKIP_MANIFEST_VALIDATION:-}" ]; then
  ./scripts/validate-manifests.sh
else
  echo "Skipping manifest validation (SKIP_MANIFEST_VALIDATION set)"
fi

echo "Pre-commit checks passed."
