Title: Backup & Restore / Recovery Flow

Description
------------
(Prepared from docs/issues/0003-implement-restore.md)

Implement documented and tested backup & restore flows for RocksDB-based services (audit DB and others). Provide commands and automated test to perform a full restore into a test environment.

Files / Places
--------------
- Storage code paths (RocksDB helpers)
- Retention runner documentation

Acceptance Criteria
-------------------
- Backup script documented and tested
- Restore script that reproduces a working environment
- Recovery tests (integration)

Labels: backup, restore, critical
Estimate: 3-5 PT

---
(Prepared locally; run `scripts/create_issues_from_md.sh` with `gh` CLI to open on GitHub)