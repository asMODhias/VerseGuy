# Copilot Rules — Extract (Critical)

## Absolute Prohibitions

- NEVER create mock data, stubs, or placeholders.
- NEVER invent architecture or features not defined in the specification.
- ALWAYS implement real, working code with error handling and logging.
- ALWAYS write tests for every function and verify compilation.

## Implementation Requirements

- Use RocksDB for storage, libp2p for P2P, Argon2 + JWT for auth.
- No cloud dependencies, local development only.
- Follow testing and code-quality rules: cargo fmt, cargo clippy, unit tests, descriptive commits.

## Workflow

- Build and run `./scripts/build.sh`, `./scripts/test.sh` frequently.
- Ensure production-ready quality from day one (no technical debt).

---
Source: `VERSEGUY_COPILOT_COMPLETE.md`
