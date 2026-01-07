# Changelog

All notable changes to this project will be documented in this file.

## Unreleased (2026-01-07)

### Added
- crates/infrastructure/auth: Authentication infrastructure scaffolded (user models, `StorageUserRepository`, session store backed by `verseguy_storage_infra`, OAuth client abstraction) and unit + integration tests added. (PR #9)
- .github/workflows/auth-integration.yml: CI job to run `cargo test -p verseguy_auth --tests` and `cargo clippy`.

### Changed
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 6 (Authentication) updated to reflect implemented auth features and next steps.

### Fixed
- N/A

### Notes
- Local full-workspace tests and clippy were executed locally and passed.

---

(Keep this file up to date as part of the Release Checklist in `docs/RELEASE_CHECKLIST.md`.)