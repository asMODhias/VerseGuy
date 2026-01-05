param()

$ErrorActionPreference = 'Stop'
$Root = git rev-parse --show-toplevel
Set-Location $Root

$DBDir = "data/scunpacked/db"
New-Item -ItemType Directory -Force -Path $DBDir | Out-Null

cargo run --bin importer -p plugins-adapter-scunpacked -- --file plugins/adapters/scunpacked/tests/fixtures/sample_ships.json --db $DBDir

Write-Host "Seeding finished. DB at $DBDir"
