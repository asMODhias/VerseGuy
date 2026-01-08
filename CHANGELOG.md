# Changelog

All notable changes to this project will be documented in this file.

## Unreleased (2026-01-08)

### Added
- crates/infrastructure/auth: Authentication infrastructure scaffolded (user models, `StorageUserRepository`, session store backed by `verseguy_storage_infra`, OAuth client abstraction) and unit + integration tests added. (PR #9)
- .github/workflows/auth-integration.yml: CI job to run `cargo test -p verseguy_auth --tests` and `cargo clippy`.
- crates/domain/fleet: Fleet domain implementation (entities, repository, RocksDB storage adapter, FleetService) with unit and integration tests. (PR: TEIL 10)
- Playwright E2E: `ui/web/e2e/fleet.spec.ts` added (Create fleet → Add ship → Verify) to cover Fleet flows in CI.
- crates/domain/operations: Operations domain implementation (entities, repository, RocksDB adapter, OperationsService) with unit and integration tests. (PR: TEIL 11)
- Playwright E2E: `ui/web/e2e/operations.spec.ts` added (Create operation → Add participant → Update status → Verify) to cover Operations flows in CI.- crates/domain/application: Application services implemented (CQRS skeleton, AppAggregate, repository, RocksDB adapter, ApplicationService) with unit and integration tests. (PR: TEIL 12)
- Master-server: Added Application APIs (`POST /v1/apps`, `GET /v1/apps`, `GET /v1/apps/{id}`) and Playwright E2E `ui/web/e2e/application.spec.ts` for TEIL 12.
- Application enhancements (TEIL 12 extension): Added support for `metadata` (k/v pairs) and `tags` on `AppAggregate`; implemented PATCH endpoint to update `name`, `metadata` and `tags` (`PATCH /v1/apps/{id}`) and added integration tests to validate metadata/tags behavior. (PR: TEIL 12 extension)
- Next (planned): Bulk operations for applications (bulk create/update/delete) and E2E coverage to be added in a follow-up PR.
### Changed
- master-server: Added Fleet HTTP handlers (`POST /v1/fleets`, `GET /v1/fleets`, `GET /v1/fleets/{id}`, `POST /v1/fleets/{id}/ships`) and integration test `master-server/tests/fleet_http_tests.rs`.
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 6 (Authentication) updated to reflect implemented auth features and next steps.
- VERSEGUY_ENTERPRISE_GUIDE.md: TEIL 10 status updated to reflect implemented Fleet domain and E2E tests.

### Fixed
- N/A

### Notes
- Local full-workspace tests and Playwright checks were executed locally (Playwright tests skip when services are unavailable). CI will now run Playwright tests and surface failures.

---

(Keep this file up to date as part of the Release Checklist in `docs/RELEASE_CHECKLIST.md`.)