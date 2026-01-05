#!/usr/bin/env bash
set -euo pipefail

# Run 'build-rust' job locally with act using the locally built runner image
act -j build-rust -W .github/workflows/ci.yml -P ubuntu-latest=verseguy/act-runner:node18 --action-offline-mode "$@"
# helper to run adapters-tests
alias act-adapters='act -j adapters-tests -W .github/workflows/ci.yml -P ubuntu-latest=verseguy/act-runner:node18 --action-offline-mode'
