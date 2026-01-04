Param()
$ErrorActionPreference = 'Stop'
Push-Location -LiteralPath (git rev-parse --show-toplevel)

Write-Host "Running pre-commit (PowerShell) checks..."
# Formatting check
& cargo fmt --all -- --check
# Lint (may be slow) - best effort
& cargo clippy --all-targets -- -D warnings
# Ensure tests compile (no run)
& cargo test --no-run --workspace

# Optional: list C++ tests if build exists
if (Test-Path "core/build") {
  if (Get-Command ctest -ErrorAction SilentlyContinue) {
    ctest -N | Out-Host
  }
}

# Validate manifests (optional: set SKIP_MANIFEST_VALIDATION=1 to skip locally)
Write-Host "-> validate manifests"
if (-not $env:SKIP_MANIFEST_VALIDATION) {
  & cargo run -p manifest-validator --
} else {
  Write-Host "Skipping manifest validation (SKIP_MANIFEST_VALIDATION set)"
}

Pop-Location
