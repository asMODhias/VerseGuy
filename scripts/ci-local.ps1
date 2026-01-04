# Local CI script to mimic GitHub Actions checks (Windows PowerShell)
# Usage: .\scripts\ci-local.ps1

Set-StrictMode -Version Latest

Write-Host "Running local CI checks..."

# 1) Rust fmt
Write-Host "-> cargo fmt --all -- --check"
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { throw "cargo fmt failed" }

# 2) Rust clippy
Write-Host "-> cargo clippy --all-targets -- -D warnings"
cargo clippy --all-targets -- -D warnings
if ($LASTEXITCODE -ne 0) { throw "cargo clippy failed" }

# 3) Run cargo tests
Write-Host "-> cargo test --workspace"
cargo test --workspace --verbose
if ($LASTEXITCODE -ne 0) { throw "cargo test failed" }

# 4) Build C++ core via CMake
Write-Host "-> cmake build (core)"
if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) {
    Write-Host "CMake not found in PATH, skipping core build" -ForegroundColor Yellow
} else {
    cmake -S core -B core/build -DCMAKE_BUILD_TYPE=Release
    if ($LASTEXITCODE -ne 0) { throw "cmake configure failed" }
    cmake --build core/build --config Release
    if ($LASTEXITCODE -ne 0) { throw "cmake build failed" }

    # Run C++ tests via CTest if available
    if (Get-Command ctest -ErrorAction SilentlyContinue) {
        Write-Host "-> ctest --output-on-failure -C Release"
        Push-Location core/build
        ctest --output-on-failure -C Release
        if ($LASTEXITCODE -ne 0) { throw "C++ tests failed" }
        Pop-Location
    } else {
        Write-Host "ctest not found, skipping C++ tests" -ForegroundColor Yellow
    }
}

# 5) Build WinUI (if dotnet is available)
Write-Host "-> dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release"
if (-not (Get-Command dotnet -ErrorAction SilentlyContinue)) {
    Write-Host "dotnet CLI not found, skipping WinUI build" -ForegroundColor Yellow
} else {
    Write-Host "dotnet --info (diagnostic)"
    dotnet --info

    # Try to build the UI project; non-fatal in local CI unless explicitly requested
    dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release -v:minimal
    $dotnetExit = $LASTEXITCODE
    if ($dotnetExit -ne 0) {
        Write-Host "Warning: dotnet build failed with exit code $dotnetExit. This may be due to missing Windows SDK / Windows App SDK or unsupported RIDs on this machine." -ForegroundColor Yellow
        Write-Host "To enforce failure in CI, set environment variable CI_STRICT_UI_BUILD=1" -ForegroundColor Yellow
        if ($env:CI_STRICT_UI_BUILD -eq '1') {
            throw "dotnet build failed"
        } else {
            Write-Host "Continuing despite UI build failure (non-fatal in local CI)." -ForegroundColor Yellow
        }
    }
}

Write-Host "Local CI checks passed." -ForegroundColor Green
