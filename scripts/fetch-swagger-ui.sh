#!/usr/bin/env bash
set -euo pipefail

# Download the official swagger-ui-dist files into crates/api/static/swagger-ui/
# Adjust VERSION as desired; check https://www.npmjs.com/package/swagger-ui-dist
VERSION="4.18.3"
OUTDIR="$(dirname "$0")/../crates/api/static/swagger-ui"
mkdir -p "$OUTDIR"

echo "Downloading swagger-ui-dist@$VERSION into $OUTDIR"
curl -sSfL "https://unpkg.com/swagger-ui-dist@$VERSION/swagger-ui-bundle.js" -o "$OUTDIR/swagger-ui-bundle.min.js"
curl -sSfL "https://unpkg.com/swagger-ui-dist@$VERSION/swagger-ui-standalone-preset.js" -o "$OUTDIR/swagger-ui-standalone-preset.min.js"
curl -sSfL "https://unpkg.com/swagger-ui-dist@$VERSION/swagger-ui.css" -o "$OUTDIR/swagger-ui.bundle.css"

echo "Done. You may need to update the docs page to use the minified bundle/preset if desired."
chmod +x "$OUTDIR/swagger-ui-bundle.min.js" || true
