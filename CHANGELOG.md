# Changelog

All notable changes to this project will be documented in this file.

## Unreleased (2026-01-07)

### Added
- crates/infrastructure/auth: Authentication infrastructure scaffolded (user models, `StorageUserRepository`, session store backed by `verseguy_storage_infra`, OAuth client abstraction) and unit + integration tests added. (PR #9)
- .github/workflows/auth-integration.yml: CI job to run `cargo test -p verseguy_auth --tests` and `cargo clippy`.
- Vendored `libp2p-swarm v0.47.0` (patched) and pinned `lru` to `v0.16.x` to address dependency compatibility/security; added minimal compatibility shims (`patches/libp2p-swarm/src/compat.rs`) exposing a small surface (re-exported `UpgradeError`, `ConnectionId` alias, `NameAsBytes`/`NameWrap`) and applied focused, localized fixes across vendored/related crates to silence warnings and keep API churn minimal. Vendored crate unit tests passed (20 passed; doc-tests: 5 passed, 3 ignored) and full-workspace tests passed locally.

### Changed
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 6 (Authentication) updated to reflect implemented auth features and next steps.

### Fixed
- N/A

### Notes
- Local full-workspace tests and clippy were executed locally and passed.

---

(Keep this file up to date as part of the Release Checklist in `docs/RELEASE_CHECKLIST.md`.)