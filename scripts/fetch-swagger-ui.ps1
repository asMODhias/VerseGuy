# PowerShell script to download swagger-ui-dist files into crates/api/static/swagger-ui
param(
    [string]$Version = '4.18.3'
)

$OutDir = Join-Path -Path $PSScriptRoot -ChildPath '..\crates\api\static\swagger-ui' | Resolve-Path -Relative
New-Item -ItemType Directory -Path $OutDir -Force | Out-Null

Write-Host "Downloading swagger-ui-dist@$Version into $OutDir"
Invoke-WebRequest -Uri "https://unpkg.com/swagger-ui-dist@$Version/swagger-ui-bundle.js" -OutFile (Join-Path $OutDir 'swagger-ui-bundle.min.js') -UseBasicParsing
Invoke-WebRequest -Uri "https://unpkg.com/swagger-ui-dist@$Version/swagger-ui-standalone-preset.js" -OutFile (Join-Path $OutDir 'swagger-ui-standalone-preset.min.js') -UseBasicParsing
Invoke-WebRequest -Uri "https://unpkg.com/swagger-ui-dist@$Version/swagger-ui.css" -OutFile (Join-Path $OutDir 'swagger-ui.bundle.css') -UseBasicParsing
Write-Host "Done."
