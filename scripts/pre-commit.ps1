Param()
$ErrorActionPreference = 'Stop'
Push-Location -LiteralPath (git rev-parse --show-toplevel)

Write-Host "Running scripts/pre-commit.ps1 checks..."

# 1) Formatting
& cargo fmt --all -- --check

# 2) Lint
& cargo clippy --all-targets -- -D warnings

# 3) Ensure tests compile (no run)
& cargo test --no-run --workspace

# 4) List C++ tests if build exists
if (Test-Path "core\build") {
  if (Get-Command ctest -ErrorAction SilentlyContinue) {
    ctest -N | Out-Host
  }
}

# 5) Validate plugin manifests (optional: set SKIP_MANIFEST_VALIDATION=1 to skip locally)
Write-Host "-> validate manifests"
if (-not $env:SKIP_MANIFEST_VALIDATION) {
  & cargo run -p manifest-validator --
} else {
  Write-Host "Skipping manifest validation (SKIP_MANIFEST_VALIDATION set)"
}

Pop-Location
