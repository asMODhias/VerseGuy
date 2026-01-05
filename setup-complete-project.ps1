param(
    [switch]$Force
)

Write-Output "🚀 Setting up Verse Guy v2.0 project (PowerShell)..."

# Helper: create directory if not exists
function EnsureDir([string]$path){
    if (-not (Test-Path -Path $path)){
        New-Item -ItemType Directory -Path $path -Force | Out-Null
        Write-Output "📁 Created: $path"
    }
}
# Backwards-compatible wrapper named with hyphen (so existing calls in docs/scripts work)
function Ensure-Dir([string]$path){
    EnsureDir $path
}

# Main directories
Ensure-Dir "core\include"; Ensure-Dir "core\src"; Ensure-Dir "core\tests"
Ensure-Dir "launcher\src"

# Containers
Ensure-Dir "containers\auth\src"; Ensure-Dir "containers\auth\tests"
Ensure-Dir "containers\storage\src"; Ensure-Dir "containers\storage\tests"
Ensure-Dir "containers\licensing\src"; Ensure-Dir "containers\licensing\tests"
Ensure-Dir "containers\compliance\src"; Ensure-Dir "containers\compliance\tests"
Ensure-Dir "containers\p2p\src"; Ensure-Dir "containers\p2p\tests"
Ensure-Dir "containers\audit\src"; Ensure-Dir "containers\audit\tests"

# Plugins
Ensure-Dir "plugins\registry\src"; Ensure-Dir "plugins\registry\tests"
Ensure-Dir "plugins\base\organization\src"; Ensure-Dir "plugins\base\organization\tests"
Ensure-Dir "plugins\base\fleet\src"; Ensure-Dir "plugins\base\fleet\tests"
Ensure-Dir "plugins\base\operations\src"; Ensure-Dir "plugins\base\operations\tests"
Ensure-Dir "plugins\pro\treasury\src"; Ensure-Dir "plugins\pro\treasury\tests"
Ensure-Dir "plugins\pro\recruitment\src"; Ensure-Dir "plugins\pro\recruitment\tests"
Ensure-Dir "plugins\enterprise\rbac\src"; Ensure-Dir "plugins\enterprise\rbac\tests"
Ensure-Dir "plugins\adapters\rsi\src"; Ensure-Dir "plugins\adapters\rsi\tests"
Ensure-Dir "plugins\adapters\discord\src"; Ensure-Dir "plugins\adapters\discord\tests"

# UI
Ensure-Dir "ui\native\Startup"; Ensure-Dir "ui\native\Auth"; Ensure-Dir "ui\native\Shell"
Ensure-Dir "ui\web\src\tabs"; Ensure-Dir "ui\web\src\components"; Ensure-Dir "ui\web\src\hooks"; Ensure-Dir "ui\web\src\utils"
Ensure-Dir "ui\web\public"

# Master Server
Ensure-Dir "master-server\src"; Ensure-Dir "master-server\tests"; Ensure-Dir "master-server\src\modules"

# Scripts
Ensure-Dir "scripts"

# Documentation
Ensure-Dir "docs\architecture"; Ensure-Dir "docs\api"; Ensure-Dir "docs\user"; Ensure-Dir "docs\developer"

# Legal
Ensure-Dir "legal"

# Installer
Ensure-Dir "installer\windows"; Ensure-Dir "installer\linux"; Ensure-Dir "installer\macos"

# Backup existing files if present
if (Test-Path -Path ".gitignore" -PathType Leaf -ErrorAction SilentlyContinue -and -not $Force){
    $bak = ".gitignore.bak.$((Get-Date -UFormat %s))"
    Copy-Item -Path ".gitignore" -Destination $bak -Force
    Write-Output "🔀 Backed up .gitignore -> $bak"
}
if (Test-Path -Path "README.md" -PathType Leaf -ErrorAction SilentlyContinue -and -not $Force){
    $bak = "README.md.bak.$((Get-Date -UFormat %s))"
    Copy-Item -Path "README.md" -Destination $bak -Force
    Write-Output "🔀 Backed up README.md -> $bak"
}

# Create .gitignore
@'
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
'@ | Set-Content -Path ".gitignore"
Write-Output "✅ .gitignore created"

# Create README
@'
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
'@ | Set-Content -Path "README.md"
Write-Output "✅ README.md created"

Write-Output "🎉 Project setup complete!"
Write-Output "Next steps:"
Write-Output "  1. cd into project directory"
Write-Output "  2. Run: git init"
Write-Output "  3. Continue with TEIL 2 (Core DLL)"