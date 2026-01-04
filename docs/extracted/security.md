# Security & Compliance — Extract

## Capability System (Zero Trust)

- Default: no permissions
- Plugins declare capabilities in manifest
- User must approve during install
- Core enforces capabilities at runtime; revocation supported

## Plugin Sandboxing

- Prefer WASM (memory-isolated)
- Native DLLs require signing and explicit user warning

## Code & Data Signing

- Core, containers, official plugins: VerseGuy certificate
- Signatures verified on load; revocation list checked

## Audit Logging (Hash-chained)

- Append-only entries with previous-hash
- Integrity verified via hash chain
- Example `AuditEntry` uses Sha256 and stored in RocksDB

## Compliance

- GDPR/CCPA/DSA support with export & deletion APIs
- Per-country rules and retention policies

---
Source: `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`
