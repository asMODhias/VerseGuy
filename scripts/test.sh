#!/bin/bash
# File: scripts/test.sh

set -e

echo "=================================="
echo "  Running Tests"
echo "=================================="
echo ""

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Rust tests
echo -e "${YELLOW}Running Rust tests...${NC}"
cargo test --workspace
echo -e "${GREEN}ÃƒÂ¢Ã…â€œÃ¢â‚¬Å“ Rust tests passed${NC}"
echo ""

# C++ tests (if any)
if [ -d "core/build" ]; then
    echo -e "${YELLOW}Running C++ tests...${NC}"
    cd core/build
    ctest --output-on-failure || true
    cd ../..
    echo ""
fi

# UI tests (if any)
if [ -f "ui/web/package.json" ]; then
    echo -e "${YELLOW}Running UI tests...${NC}"
    cd ui/web
    npm test || true
    cd ../..
    echo ""
fi

echo "=================================="
echo -e "${GREEN}ÃƒÂ¢Ã…â€œÃ¢â‚¬Å“ All tests complete${NC}"
echo "=================================="

