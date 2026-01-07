#!/usr/bin/env bash
set -euo pipefail

# Rebuild project images: act-runner and p2p

echo "Building verseguy/act-runner:node18"
docker build -t verseguy/act-runner:node18 -f scripts/dockerfiles/act-runner-node18.Dockerfile .

echo "Building verseguy/p2p:local (this can be big)
docker build -t verseguy/p2p:local -f containers/p2p/Dockerfile .

