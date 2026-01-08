Title: Add tests to verify extracted/exported data

Description
------------
(Prepared from docs/issues/0004-verify-extracted-data.md)

Add validation tests for export endpoints (GDPR export) to assert completeness and schema correctness of exported payloads.

Files / Places
--------------
- `containers/audit` export code & tests
- `master-server` GDPR export integration tests

Acceptance Criteria
-------------------
- Test coverage that ensures exported events include `audit_delete` events, correct counts, and expected fields
- Clear check for schema validity (JSON Schema or serde model assertions)

Labels: test, compliance
Estimate: 1-2 PT

---
(Prepared locally; run `scripts/create_issues_from_md.sh` with `gh` CLI to open on GitHub)