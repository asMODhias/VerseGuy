#!/bin/bash
# File: scripts/dev.sh

set -e

echo "=================================="
echo "  Development Mode"
echo "=================================="
echo ""
# Start file watchers
echo "Starting file watchers..."
echo ""

# Watch Rust code
cargo watch -x "build --workspace" &
CARGO_PID=$!

# Watch React code
cd ui/web
npm run dev &
VITE_PID=$!
cd ../..

echo "Development servers running:"
echo "  - Rust: PID $CARGO_PID"
echo "  - React: PID $VITE_PID"
echo ""
echo "Press Ctrl+C to stop all services"

# Trap Ctrl+C
trap "kill $CARGO_PID $VITE_PID 2>/dev/null" EXIT INT TERM
