# GDPR Data Processing Addendum (DPA) — Template

This is a template DPA for Verse Guy. **Legal team must review and provide final text** before publishing.

## Parties
- Data Controller: [Organization]
- Data Processor: Verse Guy (local runtime & optional Master Server services)

## Purpose & Scope
- Purpose: Provide the Verse Guy service, manage accounts, provide plugin verification, and optional cloud-assisted features.
- Types of personal data processed: user identifiers, contact information, profile metadata, audit logs (when configured), minimal telemetry (if opted-in).

## Data Subject Rights
- Access, rectification, erasure, restriction, portability, objection — implemented through API endpoints and local UI (Settings → Privacy).
- Process for handling requests: record request → authenticate requestor → export or delete data → confirm completion.

## Subprocessors
- List subprocessors (e.g., analytics, cloud storage) if any — must be documented and approved.

## Retention
- Default local retention policy: configurable per container/plugin; default retention settings for audit logs documented in `docs/`.

## Security
- Encryption at rest (AES-256) and in transit (HTTPS/TLS).
- Access control and logging of administrative operations; audit trail retention per country rules.

## Audit & Compliance
- Verse Guy team will provide audit evidence on request; legal team to provide final commitments in SLA.

---

**Next steps:** Legal team to finalize language, review subprocessors, and confirm retention times for each jurisdiction.