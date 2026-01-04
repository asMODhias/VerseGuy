# Container Developer Guide

This guide covers container development best practices (Auth, Storage, Licensing, Compliance, P2P, Audit).

Guidelines:

- Keep containers focused and small; avoid adding feature logic into the container core.
- Write clear public APIs with input validation and logging.
- Add unit tests and integration tests; follow workspace test conventions.
- Document public APIs and configuration in `docs/developer/API_REFERENCE.md`.

Build & test:

- `cargo test` inside the container crate
- Use `scripts/test.sh` to run workspace tests

Security:

- Do not log secrets; use secure storage and key derivation for sensitive data.
- All exported functions should check capabilities when acting on behalf of a plugin or user.
