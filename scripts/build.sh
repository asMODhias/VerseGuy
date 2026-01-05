#!/usr/bin/env bash
set -euo pipefail

# Cross-platform build wrapper for local development (POSIX)
# Mirrors the checks done by scripts/ci-local.ps1 and CI workflows.
# Usage: ./scripts/build.sh

echo "Running scripts/build.sh - cross-platform build wrapper"

# 1) Rust fmt
if command -v cargo >/dev/null 2>&1; then
  echo "-> cargo fmt --all -- --check"
  cargo fmt --all -- --check
else
  echo "cargo not found, skipping Rust format" >&2
fi

# 2) Rust clippy
if command -v cargo >/dev/null 2>&1; then
  echo "-> cargo clippy --all-targets --all-features -- -D warnings"
  cargo clippy --all-targets --all-features -- -D warnings
else
  echo "cargo not found, skipping clippy" >&2
fi

# 3) Run cargo tests
if command -v cargo >/dev/null 2>&1; then
  echo "-> cargo test --workspace --verbose"
  cargo test --workspace --verbose
else
  echo "cargo not found, skipping cargo test" >&2
fi

# 4) Build C++ core via CMake
if command -v cmake >/dev/null 2>&1; then
  echo "-> cmake -S core -B core/build -DCMAKE_BUILD_TYPE=Release"
  cmake -S core -B core/build -DCMAKE_BUILD_TYPE=Release
  cmake --build core/build --config Release

  if command -v ctest >/dev/null 2>&1; then
    echo "-> ctest --output-on-failure -C Release"
    (cd core/build && ctest --output-on-failure -C Release)
  else
    echo "ctest not found, skipping C++ tests" >&2
  fi
else
  echo "CMake not found, skipping core build" >&2
fi

# 5) Build WinUI (if dotnet is available)
if command -v dotnet >/dev/null 2>&1; then
  echo "-> dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release"
  dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release
else
  echo "dotnet CLI not found, skipping WinUI build" >&2
fi

# 6) Docker: check daemon and build container images
if command -v docker >/dev/null 2>&1; then
  echo "-> docker info (verify daemon)"
  docker info >/dev/null 2>&1 || { echo 'Docker daemon not running or not accessible' >&2; exit 1; }

  echo "-> Pull base images"
  docker pull rust:1.70-slim-bullseye
  docker pull debian:bullseye-slim

  echo "-> Build containers/p2p image (with progress)"
  echo "-> invoking scripts/docker-build-wrapper.sh -t verseguy/p2p:local -f containers/p2p/Dockerfile ."
  ./scripts/docker-build-wrapper.sh -t verseguy/p2p:local -f containers/p2p/Dockerfile .
else
  echo "docker CLI not found, skipping container image builds" >&2
fi

echo "Build wrapper completed."
