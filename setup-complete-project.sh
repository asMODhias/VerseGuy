#!/bin/bash
# File: setup-complete-project.sh
# Erstellt KOMPLETTE Verzeichnis-Struktur

set -e  # Exit on error

echo "🚀 Setting up Verse Guy v2.0 project..."

# Main directories
mkdir -p core/{include,src,tests}
mkdir -p launcher/src

# Containers
mkdir -p containers/auth/{src,tests}
mkdir -p containers/storage/{src,tests}
mkdir -p containers/licensing/{src,tests}
mkdir -p containers/compliance/{src,tests}
mkdir -p containers/p2p/{src,tests}
mkdir -p containers/audit/{src,tests}

# Plugins
mkdir -p plugins/registry/{src,tests}
mkdir -p plugins/base/organization/{src,tests}
mkdir -p plugins/base/fleet/{src,tests}
mkdir -p plugins/base/operations/{src,tests}
mkdir -p plugins/pro/treasury/{src,tests}
mkdir -p plugins/pro/recruitment/{src,tests}
mkdir -p plugins/enterprise/rbac/{src,tests}
mkdir -p plugins/adapters/rsi/{src,tests}
mkdir -p plugins/adapters/discord/{src,tests}

# UI
mkdir -p ui/native/{Startup,Auth,Shell}
mkdir -p ui/web/src/{tabs,components,hooks,utils}
mkdir -p ui/web/public

# Master Server
mkdir -p master-server/{src,tests}
mkdir -p master-server/src/modules

# Scripts
mkdir -p scripts

# Documentation
mkdir -p docs/{architecture,api,user,developer}

# Legal
mkdir -p legal

# Installer
mkdir -p installer/{windows,linux,macos}

echo "✅ Directory structure created"

# Create .gitignore
cat > .gitignore << 'GITIGNORE'
# Rust
target/
Cargo.lock

# C++
build/
*.o
*.obj
*.dll
*.so
*.dylib
*.exe

# Node
node_modules/
dist/
.next/

# C#
bin/
obj/

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Data
*.db
data/

# Logs
*.log
GITIGNORE

echo "✅ .gitignore created"

# Create README
cat > README.md << 'README'
# Verse Guy v2.0

**Star Citizen Organization & Fleet Management Tool**

## Architecture

- **Core:** C++ DLL (minimal bootstrap ~2MB)
- **Containers:** Rust DLLs (infrastructure services)
- **Plugins:** Rust DLLs (feature modules)
- **UI:** WinUI 3 (native) + React (web dashboards)

## Build

```bash
./scripts/build.sh
```

## Test

```bash
./scripts/test.sh
```

## Status

🚧 Active Development 🚧

Current Phase: Implementation (Week 1-3)

## License

MIT License
README

echo "✅ README.md created"
echo ""
echo "🎉 Project setup complete!"
echo ""
echo "Next steps:"
echo "  1. cd into project directory"
echo "  2. Run: git init"
echo "  3. Continue with TEIL 2 (Core DLL)"

echo "Setup script written."
