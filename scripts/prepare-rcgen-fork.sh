#!/usr/bin/env bash
set -euo pipefail

# Usage: ./prepare-rcgen-fork.sh <your-github-org>
ORG=${1:-your-org}
REPO_URL="https://github.com/$ORG/rcgen.git"
BRANCH="bump-ring-0.17"

echo "Cloning fork: $REPO_URL"
if [ -d rcgen ]; then
  echo "rcgen directory already exists; remove or move it first"
  exit 1
fi

git clone "$REPO_URL"
cd rcgen

git checkout -b "$BRANCH"

# Patch Cargo.toml: set ring = "^0.17"
perl -0777 -pe "s/(ring\s*=\s*\")0\.16\.(\d+)(\")/$1^0.17$3/g" -i Cargo.toml || true

# Run tests
cargo test

echo "If tests pass, push the branch and open a PR to upstream." 

echo "git push origin $BRANCH" 
