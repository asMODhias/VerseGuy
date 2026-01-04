# Implementation TODOs — Weeks 1–4 (Actionable)

This TODO list is derived from the project specification and is intended to be an executable plan for the first 4 weeks of development.

## Immediate (Now)
- [x] Ensure repository workspace is initialized with `Cargo.toml`, core, launcher, `ui/` and `containers/` directories (verify by running `./scripts/build.sh`).
- [x] Add `docs/extracted/` and link from `README.md`.
- [x] Add CI job skeleton for local builds/testing (GitHub Actions-style + `scripts/ci-local.ps1`).
- [x] Create `containers/auth` and `containers/storage` Cargo projects.

## Week 1–2: Foundation
- Core (C++):
  - [x] Implement minimal bootstrap `VerseguY.Core.dll` (C++), export `Initialize()`.
  - [x] Implement `IPlugin.h`, `IPluginHost.h`, `Capabilities.h` interfaces.
    - [x] Add unit tests for plugin loader and integrate into CMake (build target exists; test execution integrated via CTest and CI).
- Launcher:
  - [x] Implement tiny `VerseguY.exe` stub to load core DLL. (Implemented; core is built and tested in Windows CI).
- WinUI Shell:
  - [x] Create WinUI 3 C# project skeleton with `MainWindow.xaml` and startup flow.
- Project tooling:
  - [ ] Add `scripts/build.sh`, `scripts/test.sh` (PowerShell equivalent exists: `scripts/ci-local.ps1`).

## Week 3–4: Containers & Storage
- Auth Container:
  - [x] Implement `containers/auth` crate with `LocalAuth` and `Session` modules.
  - [x] Implement registration and login methods with Argon2 hashing and JWT session creation.
  - [x] Add unit tests covering registration/login/validation/error cases.
- Storage Container:
  - [x] Implement `containers/storage` crate with `RocksDBStorage` wrapper.
  - [x] Implement typed `put`/`get` helpers and prefix scanning for export.
  - [x] Add tests for CRUD and export functionality.
- Integration:
  - [x] Wire `Auth` to `Storage` in tests (real RocksDB instance in temp dir).
  - [x] Add E2E: register user → login → create session → validate token (session persisted in RocksDB).

## Master Server & Plugins (additional tasks)
- [x] Scaffold `master-server` crate (axum) and tests
- [x] Implement `/auth/register`, `/auth/login`, `/license/validate`
- [x] Implement plugin registry persistence and endpoints: `POST /plugins/publish`, `GET /plugins/search`
- [x] Add in-process and integration tests for master-server routes

## CI & Build
- [x] GitHub Actions workflow `.github/workflows/ci.yml` (Linux + Windows job)
- [x] Local CI script `scripts/ci-local.ps1` (runs fmt/clippy/tests and optional CMake/dotnet steps)
- [x] Add `global.json` locking .NET SDK to 8.0.100 (deterministic WinUI builds)

## Remaining / Next Tasks (Prioritized)
1. Stabilize & monitor C++ tests in CI; add reporting/coverage as needed. (High)
2. Add `scripts/build.sh` and `scripts/test.sh` cross-platform wrappers that call the appropriate steps from `ci-local.ps1`. (Medium)
3. Implement plugin signing & verification flow (master-server + core) and tests. (Medium)
4. Add more E2E tests: master-server publish/search with auth and publishing tokens, license lifecycle tests. (Medium)
5. Implement WASM plugin sandboxing proof-of-concept and integration tests. (Low)

## Acceptance Criteria (for Weeks 1–4)
- `cargo test` and C++ tests pass locally and in CI (CTest used for C++ tests).
- Local auth works end-to-end with real RocksDB storage (done).
- Basic plugin interface compiled and unit-tested in C++ (headers + loader present; C++ tests integrated into CI).
- WinUI shell builds and can be executed when .NET SDK is installed (global.json added to force SDK version).
- CI job runs build & tests locally (Windows job requires .NET SDK installed locally to run WinUI build).

---
Update this file as tasks are completed or new requirements emerge.