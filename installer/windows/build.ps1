<#
Simple WiX build script for VerseguY MSI.
Requires: WiX Toolset installed and `candle.exe` / `light.exe` available in PATH.
Usage: .\build.ps1 -Configuration Release -OutputDir ..\..\target\installers
#>
param(
    [string]$Configuration = "Release",
    [string]$OutputDir = "..\..\target\installers"
)

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Definition
Push-Location $scriptDir

if (-not (Get-Command candle.exe -ErrorAction SilentlyContinue)) {
    Write-Error "WiX Toolset (candle.exe) not found in PATH. Install WiX to build MSI."
    exit 1
}

mkdir -Force $OutputDir | Out-Null

$wxs = Join-Path $scriptDir "verseguy.wxs"
$obj = Join-Path $scriptDir "verseguy.wixobj"

Write-Host "Compiling WiX source..."
candle.exe -o $obj $wxs

Write-Host "Linking MSI..."
$msi = Join-Path (Resolve-Path $OutputDir) "VerseguY-$((Get-Date).ToString('yyyyMMddHHmm')).msi"
light.exe -o $msi $obj

Write-Host "MSI created: $msi"

Pop-Location
