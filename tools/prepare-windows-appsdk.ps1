<#
Helper: Check for Windows App SDK and show install commands (does NOT install by default).
Run this on Windows as Admin to install the Windows App SDK targeting pack if missing.
#>

Write-Host "Checking for Windows App SDK targeting pack..."
$pkg = Get-PackageProvider -ListAvailable -ErrorAction SilentlyContinue

# Quick check: look for Windows App SDK targeting pack folder
$possible = "C:\Program Files\WindowsAppSDK"
if (Test-Path $possible) {
    Write-Host "Windows App SDK seems installed (found $possible)."
    exit 0
}

Write-Host "Windows App SDK not detected. To build the native UI you need the Windows App SDK targeting pack or Visual Studio workloads."
Write-Host "Options to install (run as Administrator):"
Write-Host "  - winget (recommended):"
Write-Host "      winget install --id Microsoft.WindowsAppSDK.1.4 -e"
Write-Host "  - Chocolatey:"
Write-Host "      choco install microsoft-windows-appsdk -y"
Write-Host "  - Visual Studio: install 'Desktop development with C++' and 'Windows App SDK' components via VS Installer."
Write-Host "After installation, restart your shell and run: dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release"
exit 1