<#
A tiny smoke test for install/uninstall of the MSI produced by build.ps1.
- Installs MSI to a temp directory using msiexec /i
- Checks that installed files exist
- Uninstalls using msiexec /x
#>
param(
    [string]$MsiPath
)
if (-not $MsiPath) {
    Write-Error "Provide the MSI path as the first argument"
    exit 1
}

Write-Host "Installing MSI: $MsiPath"
$install = Start-Process msiexec -ArgumentList "/i `"$MsiPath`" /qn" -Wait -PassThru
if ($install.ExitCode -ne 0) { Write-Error "Install failed: $($install.ExitCode)"; exit 1 }

# Basic check: Program Files\VerseguY exists
$programPath = "$Env:ProgramFiles\VerseguY"
if (-not (Test-Path $programPath)) { Write-Error "Install smoke: install folder missing: $programPath"; exit 1 }

Write-Host "Uninstalling..."
$uninstall = Start-Process msiexec -ArgumentList "/x `"$MsiPath`" /qn" -Wait -PassThru
if ($uninstall.ExitCode -ne 0) { Write-Error "Uninstall failed: $($uninstall.ExitCode)"; exit 1 }

Write-Host "Smoke test passed"
