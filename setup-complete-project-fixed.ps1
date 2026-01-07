param(
    [switch]$Force
)

Write-Output "Setting up Verse Guy v2.0 project (PowerShell) - cleaned version"

$dirs = @(
    "core\include","core\src","core\tests",
    "launcher\src",
    "containers\auth\src","containers\auth\tests",
    "containers\storage\src","containers\storage\tests",
    "containers\licensing\src","containers\licensing\tests",
    "containers\compliance\src","containers\compliance\tests",
    "containers\p2p\src","containers\p2p\tests",
    "containers\audit\src","containers\audit\tests",
    "plugins\registry\src","plugins\registry\tests",
    "plugins\base\organization\src","plugins\base\organization\tests",
    "plugins\base\fleet\src","plugins\base\fleet\tests",
    "plugins\base\operations\src","plugins\base\operations\tests",
    "plugins\pro\treasury\src","plugins\pro\treasury\tests",
    "plugins\pro\recruitment\src","plugins\pro\recruitment\tests",
    "plugins\enterprise\rbac\src","plugins\enterprise\rbac\tests",
    "plugins\adapters\rsi\src","plugins\adapters\rsi\tests",
    "plugins\adapters\discord\src","plugins\adapters\discord\tests",
    "ui\native\Startup","ui\native\Auth","ui\native\Shell",
    "ui\web\src\tabs","ui\web\src\components","ui\web\src\hooks","ui\web\src\utils",
    "ui\web\public",
    "master-server\src","master-server\tests","master-server\src\modules",
    "scripts",
    "docs\architecture","docs\api","docs\user","docs\developer",
    "legal",
    "installer\windows","installer\linux","installer\macos"
)

foreach ($d in $dirs) {
    if (-not (Test-Path -Path $d)) {
        New-Item -ItemType Directory -Path $d -Force | Out-Null
        Write-Output "Created: $d"
    }
}

# Backup existing files if present
if ((Test-Path -Path '.gitignore' -PathType Leaf) -and -not $Force) {
    $bak = ".gitignore.bak.$((Get-Date -UFormat %s))"
    Copy-Item -Path '.gitignore' -Destination $bak -Force
    Write-Output "Backed up .gitignore -> $bak"
}
if ((Test-Path -Path 'README.md' -PathType Leaf) -and -not $Force) {
    $bak = "README.md.bak.$((Get-Date -UFormat %s))"
    Copy-Item -Path 'README.md' -Destination $bak -Force
    Write-Output "Backed up README.md -> $bak"
}

# Create .gitignore
$gitignore = @(
"# Rust",
"target/",
"Cargo.lock",
"",
"# C++",
"build/",
"*.o",
"*.obj",
"*.dll",
"*.so",
"*.dylib",
"*.exe",
"",
"# Node",
"node_modules/",
"dist/",
".next/",
"",
"# C#",
"bin/",
"obj/",
"",
"# IDE",
".vscode/",
".idea/",
"*.swp",
"*.swo",
"",
"# OS",
".DS_Store",
"Thumbs.db",
"",
"# Data",
"*.db",
"data/",
"",
"# Logs",
"*.log"
)
$gitignore | Set-Content -Path '.gitignore' -Encoding UTF8
Write-Output "Created .gitignore"

# Create README
$readme = @(
"# Verse Guy v2.0",
"",
"Star Citizen Organization & Fleet Management Tool",
"",
"## Architecture",
"",
"- Core: C++ DLL",
"- Containers: Rust DLLs",
"- Plugins: Rust modules",
"- UI: WinUI 3 + React",
"",
"## Build",
"",
"./scripts/build.sh",
"",
"## Test",
"",
"./scripts/test.sh",
"",
"## License",
"",
"MIT"
)
$readme | Set-Content -Path 'README.md' -Encoding UTF8
Write-Output "Created README.md"

Write-Output "Project setup complete. Next steps: 1) cd into project directory 2) git init 3) Continue with TEIL 2 (Core DLL)"
