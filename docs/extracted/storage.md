# Storage — RocksDB (Extract)

## RocksDB Wrapper (Canonical)

Use RocksDB as the local persistent store for users, sessions, plugins, audit logs, and plugin data.

Key implementation points:

- Use RocksDB Options: create_if_missing=true.
- Serialize values as JSON (serde) for schema evolution.
- Provide typed put/get helpers with generic Serialize/Deserialize.
- Provide prefix scans for export (GDPR portability).

```rust
pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()> where V: Serialize { ... }
    pub fn get<K, V>(&self, key: K) -> Result<Option<V>> where V: Deserialize { ... }
}
```

Backup & Encryption:

- Encrypted at rest (AES-256) per spec.
- Exports for GDPR use prefix scanning and filtering.

---
Source: `VERSEG_UY_COPILOT_COMPLETE.md` and `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`
