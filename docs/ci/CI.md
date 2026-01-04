# Continuous Integration — Overview

This directory documents the CI workflows used in the repository (.github/workflows).

Primary workflows:

- `ci.yml` — builds Rust and C++ components, runs tests across platforms (Ubuntu, Windows), runs admin CLI smoke tests, and includes a `docs-validate` job.
- `security-scan.yml` — vulnerability scanning (placeholder, see `docs/ci/SECURITY_SCAN.md`).
- `compliance-check.yml` — legal/compliance checks (placeholder, see `docs/ci/COMPLIANCE_CHECK.md`).

Local testing:

- Use `scripts/ci-local.ps1` (Windows) for running the essential CI steps locally.
- Rust build + tests: `./scripts/test.sh` or `cargo test --workspace`.

Artifacts and logs:

- Test logs are uploaded by the workflows for debugging.

Next steps:

- Harden `security-scan.yml` and `compliance-check.yml` with concrete tools (cargo-audit, SCA, DLP checks).
- Add optional manual jobs for network-sensitive tests (mdns_stress) as manual/allow_failure jobs.
