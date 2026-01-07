#!/usr/bin/env bash
set -euo pipefail

# Build retention_runner binary and docker image locally
# Usage: ./build-retention-runner.sh <image-tag>

IMAGE_TAG=${1:-local}
REPO_OWNER=${GITHUB_ORG:-your-org}
IMAGE_NAME=ghcr.io/${REPO_OWNER}/verseguy-audit-runner:${IMAGE_TAG}

echo "Building retention_runner (release)"
cargo build -p verseguy_audit_infra --bin retention_runner --release

BIN=target/release/retention_runner
if [ ! -f "$BIN" ]; then
  echo "Binary not found: $BIN" >&2
  exit 1
fi

echo "Copying binary to crate dir"
cp "$BIN" crates/infrastructure/audit/retention_runner

echo "Building docker image: ${IMAGE_NAME}"
docker build -t "${IMAGE_NAME}" crates/infrastructure/audit

echo "Built image: ${IMAGE_NAME}"

echo "Cleaning up temporary binary"
rm crates/infrastructure/audit/retention_runner

echo "Done"
