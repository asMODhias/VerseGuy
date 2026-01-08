#!/usr/bin/env bash
set -euo pipefail

# This script creates GitHub issues from markdown stubs in docs/issues.
# It requires `gh` CLI to be authenticated and available in PATH.
# Run: ./scripts/create_issues_from_md.sh

MD_DIR="docs/issues"
CREATED_DIR="docs/created_issues"

mkdir -p "$CREATED_DIR"

for md in "$MD_DIR"/*.md; do
  [ -e "$md" ] || continue
  filename=$(basename -- "$md")
  echo "Processing $filename"

  title=$(sed -n '1p' "$md" | sed 's/^Title: //')
  body=$(sed '1,200p' "$md")

  if command -v gh >/dev/null 2>&1; then
    echo "Creating GitHub issue for: $title"
    gh issue create --title "$title" --body-file "$md" --label "backlog" || {
      echo "gh issue create failed for $title"
    }
  else
    echo "gh CLI not found; writing local stub: $CREATED_DIR/$filename"
    cp "$md" "$CREATED_DIR/$filename"
  fi
done

echo "Done. If gh CLI is available and authenticated, issues were created; otherwise stubs placed under $CREATED_DIR."