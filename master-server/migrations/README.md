# Master Server Migrations (Informational)

This project uses RocksDB (key-value) storage rather than a relational DB. Migrations describe the keys and initial items to be created.

Legal keys:
- `legal:doc:{type}:{version}` -> serialized `LegalDocument` (JSON)
- `legal:latest:{type}` -> serialized `LegalDocument` (JSON) (pointer to latest)
- `legal:revoked:{id}` -> serialized `Revocation` (JSON)
- `legal:acceptance:{user_id}:{doc_id}` -> serialized acceptance entry (TBD)

To make a programmatic migration, write a small script that opens the storage path and writes initial documents.
