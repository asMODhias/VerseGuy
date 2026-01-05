# Developer Setup

This file documents the recommended developer environment and common commands.

## Prerequisites

- Rust (stable) + cargo: Install via https://rustup.rs
- CMake & Visual Studio (Windows) / build-essential (Linux) for native components
- Node.js 18+ (UI)
- Git

## Basic workflow

1. Clone repo and run build/test

```bash
git clone https://github.com/OWNER/REPO.git
cd REPO
./scripts/build.sh
cargo test --workspace
```

2. Formatting & Linting

```bash
# Format all
cargo fmt --all
# Run clippy (may fail if production code contains unwraps)
cargo clippy --all-targets --all-features
```

3. Productivity commands

- Run only Rust unit tests for a crate: `cargo test -p <crate_name>`
- Run a single integration test file: `cargo test --test <test_name> -p <crate>`
- Run tests with `--nocapture` to see debug prints

## Notes on `unwrap` / `expect`

The project policy discourages `unwrap`/`expect` in production/library code. Use `?` or explicit `Result` handling. CI enforces this via `clippy::unwrap_used` (see `.github/workflows/ci.yml`).

## Windows specifics

- Use the provided PowerShell scripts in `scripts/` when available (e.g., `./scripts/build.ps1`, `./scripts/dev.ps1`).

## Updating CI badges

Update the `OWNER/REPO` placeholders in `README.md` with your repository path to enable working status badges.