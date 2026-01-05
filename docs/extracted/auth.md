# Authentication — Extract (Canonical)

## Multi-Auth Overview

Supported methods:

- Local (username + password) — Passwords hashed with Argon2; sessions issued as JWTs (24h expiry).
- OAuth Google — Access token; optional imports (Calendar, Drive).
- OAuth Discord — Server membership, role sync.
- OAuth Twitch — Stream schedule, follower counts.

Features:

- Local: Offline-only, privacy-focused, Free tier.
- OAuth: Cloud backup, cross-device sync, external integrations.

## Local Auth (Implementation Notes)

- Use Argon2 for hashing.
- Validate inputs (username length >=3, password length >=8).
- Store user records in RocksDB.
- Sessions are JWTs, include license and expiry, store session records locally.

## Session Management (Excerpt)

```rust
pub struct Session {
    pub user_id: String,
    pub auth_method: AuthMethod,
    pub license: License,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}
```

## APIs (Master Server)

- POST /auth/register
- POST /auth/login
- POST /auth/refresh
- GET  /auth/validate
- POST /auth/logout

---
Source: `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`
