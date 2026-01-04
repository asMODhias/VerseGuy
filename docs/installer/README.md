# Installer Reference

This directory contains platform packaging helpers and scripts.

Windows
- `installer/windows/verseguy.wxs` — WiX manifest (skeleton)
- `installer/windows/build.ps1` — build script (requires WiX Toolset `candle.exe`/`light.exe`)
- `scripts/install_smoke.ps1` — smoke test to install/uninstall generated MSI

Usage
- Requirements: WiX Toolset installed and available on PATH (`candle.exe`, `light.exe`).
- Build: run `powershell ./installer/windows/build.ps1 -OutputDir ..\\target\\installers` on Windows agent/host.
- Smoke test: run `powershell ./scripts/install_smoke.ps1 path\\to\\VerseguY-<timestamp>.msi` with administrator privileges.
- Signing: artifact signing is a separate step; signers must provide certificates via CI secrets. See `docs/installer/USAGE.md` for signing placeholders.

macOS
- `installer/macos/build.sh` — placeholder for DMG build and notarization

Linux
- `installer/linux/` — DEB and RPM packaging (placeholders)

CI
- Release pipeline should run packaging jobs on releases/tags and collect artifacts. Artifact signing steps must be gated behind secrets and are optional for local testing.
