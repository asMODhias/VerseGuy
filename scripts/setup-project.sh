#!/bin/bash
set -euo pipefail

# File: scripts/setup-project.sh
# Creates the recommended directory structure for VerseguY V2.0

BASE_DIR="$(dirname "${BASH_SOURCE[0]}")/.."
cd "$BASE_DIR"

echo "Creating directory structure..."
# Haupt-Verzeichnisse
mkdir -p core/{include,src,tests}
mkdir -p launcher/src
mkdir -p containers/{auth,storage,licensing,compliance,p2p,audit}/{src,tests}
mkdir -p plugins/{registry,base,pro,enterprise,adapters}/{src,tests}
mkdir -p plugins/base/{organization,fleet,operations}/{src,tests}
mkdir -p plugins/adapters/{rsi,discord,fleetyards}/{src,tests}
mkdir -p ui/{native,web}
mkdir -p ui/native/{Startup,Auth,Shell}
mkdir -p ui/web/{src,public}
mkdir -p ui/web/src/{tabs,components,hooks}
mkdir -p master-server/{src,tests}
mkdir -p master-server/src/modules
mkdir -p scripts
mkdir -p docs/{architecture,api,user,developer}
mkdir -p legal
mkdir -p installer/{windows,linux,macos}

# Helpful files
if [ ! -f README.md ]; then
  cat > README.md <<'README'
# Verse Guy v2.0

**Star Citizen Organization & Fleet Management**

## Status

ÃƒÂ°Ã…Â¸Ã…Â¡Ã‚Â§ **In Active Development** ÃƒÂ°Ã…Â¸Ã…Â¡Ã‚Â§

Current Phase: Core Implementation (Week 1-2)

## Architecture

- **Core:** C++ DLL (minimal bootstrap)
- **Containers:** Rust DLLs (infrastructure)
- **Plugins:** Rust DLLs (features)
- **UI:** WinUI 3 + React

## Build

```bash
# Build everything
./scripts/build.sh

# Run tests
./scripts/test.sh

# Development mode
./scripts/dev.sh
```

## Documentation

See `docs/` directory for complete documentation.

## License

MIT License - See LICENSE file
README
fi

echo "ÃƒÂ¢Ã…â€œÃ¢â‚¬Â¦ Verzeichnis-Struktur erstellt"

exit 0

