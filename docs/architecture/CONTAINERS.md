# Containers — Responsibilities & Layout

Verse Guy containers are small Rust DLLs implementing core infrastructure services. Each container focuses on a single responsibility and exposes a well-defined API.

Common containers (as defined in the spec):

- **auth/**: Multi-auth support (Local, Google, Discord, Twitch). Manages sessions and tokens.
- **storage/**: RocksDB local storage, cloud sync adapters, and P2P CRDT support.
- **licensing/**: License validation, feature gating (Free/Pro/Enterprise).
- **compliance/**: GDPR/CCPA/DSA helpers and ToS validator helpers.
- **p2p/**: libp2p mesh, DHT, mDNS discovery and sync primitives.
- **audit/**: Append-only audit logs, integrity verification, export functions.

Development notes:
- Containers must be small, auditable and testable.
- Prefer composition and dependency injection for testability.
- All public container APIs must include input validation, logging and tests.

See `containers/` for concrete implementations and tests.