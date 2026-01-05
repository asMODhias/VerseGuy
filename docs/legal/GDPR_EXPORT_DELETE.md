# GDPR: Export & Deletion Procedures

This document explains how the Master Server implements GDPR data portability (export) and deletion requests.

## Export: `GET /audit/export/{user_id}`

- Purpose: Provide a machine‑readable export of the user's audit events for portability/forensics.
- Returns: JSON object { entries: AuditEntry[] } where AuditEntry contains id, seq, timestamp, user_id, event, hash.
- Scope: This API returns audit entries that are explicitly tied to the user (e.g. audit.user_id == user_id).
- Authorization: Admin/operator API can call this endpoint to retrieve user exports for legal requests; the endpoint should be guarded in production (e.g., extra admin auth or approval per policy).

## Deletion: `DELETE /users/{user_id}/data`

- Purpose: Execute user data deletion requests (Right to Erasure).
- Actions performed:
  - Delete ToS acceptance keys for the user (`tos:{user_id}:*` and `tos:latest:{user_id}`).
  - Delete audit entries that have `user_id` set to the target user. Note: deleting audit entries may affect integrity checks; keep a revocation/placeholder record when required by law.
  - Any additional data removal steps must be added here (storage, sub-systems, backups) and tracked in the deletion audit.
- Response: { ok: true, deleted: <count> }
- Considerations:
  - Deletion must be logged in a way that preserves proof that deletion occurred (audit of the deletion action itself should remain with minimal metadata).
  - For backups and long-term archival, deletion should be scheduled and tracked; if backups are immutable, track legal process for purge during restore windows.

## Process & SLA

- User requests should be validated and authenticated as appropriate.
- SLA targets: respond within 45 days per GDPR; track timelines in legal ticketing system.
- Some jurisdictions or legal holds may require delaying deletion (legal hold). Ensure the system supports holding deletion requests until cleared.

## Developer Notes

- The audit exporter uses `containers/audit::AuditService::export_for_user` and returns deterministic seq ordering.
- Deletion uses `RocksDBStorage::prefix_delete` and targeted deletes for `audit:{id}` keys. Ensure delete operations are idempotent and retriable.

---

Keep this document up to date when retention rules, jurisdiction handling, or backup strategies change.
