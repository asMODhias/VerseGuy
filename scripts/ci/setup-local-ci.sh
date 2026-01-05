#!/usr/bin/env bash
set -euo pipefail

echo "Setting up local CI: pulling/building required images"

# 1) Pull act base runner image
echo "-> Pulling nektos/act-environments-ubuntu:18.04"
docker pull nektos/act-environments-ubuntu:18.04 || true

# 2) Build local act-runner with Node 18
echo "-> Building verseguy/act-runner:node18 from scripts/dockerfiles/act-runner-node18.Dockerfile"
docker build -t verseguy/act-runner:node18 -f scripts/dockerfiles/act-runner-node18.Dockerfile .

# 3) Optional: build containers/p2p (heavy). Leave commented; users can enable.
echo "Note: Building 'containers/p2p' can be very large. To build, run: ./scripts/ci/rebuild-images.sh"

echo "Setup completed. Use ./scripts/ci/run-act-job.sh to run CI jobs locally (it uses verseguy/act-runner:node18)."
