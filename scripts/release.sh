#!/bin/bash
# File: scripts/release.sh

set -e

VERSION="2.0.0"

echo "=================================="
echo "  Building Release v$VERSION"
echo "=================================="
echo ""

# Build everything
./scripts/build.sh

# Run tests
./scripts/test.sh

# Create release directory
RELEASE_DIR="release/verseguy-v$VERSION"
mkdir -p "$RELEASE_DIR"

echo "Copying binaries..."

# Copy core
cp core/build/bin/Release/VerseguY.Core.dll "$RELEASE_DIR/" || true

# Copy launcher
cp target/release/VerseguY.exe "$RELEASE_DIR/" || true

# Copy containers
mkdir -p "$RELEASE_DIR/containers"
cp target/release/verseguy_*.dll "$RELEASE_DIR/containers/" || true

# Copy plugins
mkdir -p "$RELEASE_DIR/plugins"
cp target/release/verseguy_plugin_*.dll "$RELEASE_DIR/plugins/" || true

# Copy UI
mkdir -p "$RELEASE_DIR/ui"
cp -r ui/native/bin/Release/*/* "$RELEASE_DIR/ui/" || true
cp -r ui/web/build "$RELEASE_DIR/ui/web" || true

# Copy documentation
cp README.md "$RELEASE_DIR/" || true
cp LICENSE "$RELEASE_DIR/" || true

echo ""
echo "=================================="
echo "ÃƒÂ¢Ã…â€œÃ¢â‚¬Å“ Release v$VERSION built!"
echo "=================================="
echo ""
echo "Location: $RELEASE_DIR"

