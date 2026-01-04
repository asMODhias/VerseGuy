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

Pop-Location
