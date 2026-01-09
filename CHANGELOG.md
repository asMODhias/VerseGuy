# Changelog

All notable changes to this project will be documented in this file.

## Unreleased (2026-01-07)

### Added
- crates/infrastructure/auth: Authentication infrastructure scaffolded (user models, `StorageUserRepository`, session store backed by `verseguy_storage_infra`, OAuth client abstraction) and unit + integration tests added. (PR #9)
- .github/workflows/auth-integration.yml: CI job to run `cargo test -p verseguy_auth --tests` and `cargo clippy`.
- Vendored `libp2p-swarm v0.47.0` (patched) and pinned `lru` to `v0.16.x` to address dependency compatibility/security; added minimal compatibility shims (`patches/libp2p-swarm/src/compat.rs`) exposing a small surface (re-exported `UpgradeError`, `ConnectionId` alias, `NameAsBytes`/`NameWrap`) and applied focused, localized fixes across vendored/related crates to silence warnings and keep API churn minimal. Vendored crate unit tests passed (20 passed; doc-tests: 5 passed, 3 ignored) and full-workspace tests passed locally.
- plugins/base/fleet: Added `update_ship` and `delete_ship` service methods and plugin-level tests (6 tests) to cover ship lifecycle and owner/permission checks. (PR #21)
- crates/api: Added Ship endpoints (`POST /ships`, `GET /ships/{owner_id}`, `GET/PUT/DELETE /ships/{owner_id}/{ship_id}`) and end-to-end integration test `crates/api/tests/ships.rs` (test `ships_end_to_end`); added short API docs `docs/api/fleet.md` with test setup notes. (PR #23)
- crates/application: Added `update_fleet` and `delete_fleet` methods to support integration-level flows for Fleet.
- crates/tests: Added `integration_fleet.rs` with 5 integration tests (create/get, add_ship success & invalid-fleet, update + delete); tests avoid `.unwrap()`/`.expect()` to satisfy lint policy.
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 14 - Fleet status updated to reflect new tests and progress.

### Changed
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 6 (Authentication) updated to reflect implemented auth features and next steps.

### Fixed
- N/A

### Notes
- Local full-workspace tests and clippy were executed locally and passed.

---

(Keep this file up to date as part of the Release Checklist in `docs/RELEASE_CHECKLIST.md`.)