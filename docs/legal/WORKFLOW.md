# Legal Workflow & Document Lifecycle

This document describes the legal publication workflow, versioning, revocation and the acceptance lifecycle for Terms, Privacy Policy and related legal documents.

## Overview

- Legal documents are authored in `legal/` and published via the Master Server Admin API (`POST /admin/legal`).
- Each published document is stored as a named document with **type** (e.g. `tos`, `privacy`, `dpa`) and a **version** string.
- A `latest` pointer is kept per document type (key `legal:latest:{type}`) to denote current authoritative version used by clients.
- Revocations are stored as revocation records (`legal:revoked:{id}`) for audit, and an active document can be superseded by publishing a newer version.

## Authoring

1. Draft legal text in `legal/ToS.md` (or `PrivacyPolicy.md`, `GDPR_DPA.md`).
2. Ensure the document contains metadata header: version, author, date and short change summary.
3. Run local spelling/grammar checks and internal review before publishing.

## Publishing (Admin API)

- Endpoint: `POST /admin/legal` (admin authentication required via `x-admin-token` and `MASTER_ADMIN_TOKEN`).
- Payload: { doc_type, version, title, content, author }
- On success: the document is saved under `legal:doc:{type}:{version}` and `legal:latest:{type}` is updated.
- All admin actions are audited via audit log and must include a rationale in the admin UI or the API call.

## Revocation

- Use `POST /admin/legal/{id}/revoke` to create a revocation record for a legal document (admin only).
- Revocation does NOT delete historical copies; it marks them for legal visibility and enforcement purposes.

## Acceptance lifecycle (Client & Server)

- Client must fetch `GET /legal/latest/:type` and present the document to the user on first run or when the `version` differs from the user's stored acceptance.
- Client POSTs acceptance to `POST /auth/tos` with { user_id, accepted_at, version }.
- Server stores acceptance under `tos:{user_id}:{version}` and updates `tos:latest:{user_id}` for quick lookup.
- The Master Server enforces acceptance in specific flows (e.g. plugin publish attempted by a user: must include `x-user-id` header and the server checks acceptance before proceeding).

## Audit & Retention

- All legal publishes, revocations and acceptance events are recorded in the Audit Vault.
- Retention policy: keep legal records per country/regulation requirements (see `docs/legal/GDPR_EXPORT_DELETE.md`).

## Security & Approvals

- Publishing legal texts requires admin auth and an internal legal sign-off (see Legal Review Checklist).
- All versions must be immutable once published; corrections require a new version and a documented migration note.

## Quick commands

- Create new legal doc (local): edit `legal/<DOC>.md` and run the Admin POST with `x-admin-token`.
- Revoke: `POST /admin/legal/{id}/revoke` with reason.

---

> **Note:** This is an operational document for engineers and legal reviewers. Final, public-facing legal text must be reviewed and approved by legal counsel before publishing.
