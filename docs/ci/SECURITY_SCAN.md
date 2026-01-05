# Security Scan — Guidance

This page describes the security scanning approach for the repository.

Suggested tools:

- `cargo audit` for Rust dependency vulnerabilities.
- `snyk` or `trivy` for container and dependency scanning.
- `Dependabot` (already present) for automated dependency PRs.

Workflow recommendations:

- Run dependency scans on pull requests and on a schedule.
- Fail PRs on high-severity findings and create issues for medium severity.
- Upload scan reports as artifacts for triage.
