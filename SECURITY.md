# Security Guidelines (Enterprise)

This repository follows the Enterprise Security Guide (ISO/27001 + OWASP Top 10). Key rules:

- Do not store secrets in source code. Use environment variables or secret managers (HashiCorp Vault, AWS Secrets Manager).
- Use `cargo-audit` and `cargo-deny` in CI. Deny yanked and critical advisories.
- Enforce TLS 1.3 only in outbound connections where possible.
- Input validation and output encoding for all external inputs.
- Rate limiting middleware for HTTP endpoints.
- Audit logs must be append-only and tamper-evident (`verseguy_audit` crate).

See `docs/security.md` for a comprehensive checklist.
