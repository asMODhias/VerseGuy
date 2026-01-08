# Offen: Nicht-triviale TODOs (Automatisch zusammengestellt)

Diese Datei listet die komplexeren TODO/FIXME-/UNIMPLEMENTED‑Einträge, die bei einem Repo‑Sweep gefunden wurden. Jeder Eintrag enthält eine kurze Beschreibung, betroffene Dateien, vorgeschlagene Labels und eine grobe Aufwandsschätzung. Bitte erstelle für jeden Eintrag ein GitHub‑Issue (oder verwalte via kanban) und verlinke dieses Dokument.

## 1) Implement Twitch OAuth
- Files: `containers/auth/src/oauth.rs` (TODO: Twitch impl)
- Beschreibung: Vollständigen OAuth‑Flow für Twitch implementieren (Auth URL, Token Exchange, Userinfo, state/CSRF management).
- Labels: feature, oauth, high-priority
- Aufwand: 2-4 PT
- Vorschlag: Reuse existing Google/Discord implementations as template; write integration tests (mock Twitch).

## 2) Implement Event Bus / Runtime Event Handling
- Files/Refs: `VERSEGUY_COMPLETE_IMPLEMENTATION_GUIDE.md` (TODO: event bus), multiple TODOs in event-related code
- Beschreibung: Architektur & Implementierung eines Event Buses (intra-process pub/sub or external like NATS), ensure durable delivery for audit events / retention orchestration.
- Labels: architecture, design, medium-priority
- Aufwand: 3-7 PT

## 3) Implement restore / recovery flows
- Files/Refs: multiple TODOs mentioning "restore" and "recovery" logic
- Beschreibung: Provide documented and tested restore path from RocksDB backups/exports. Automate 'replay' or reconciliation process for a subset of tables (audit events, users).
- Labels: backup, restore, critical
- Aufwand: 3-5 PT

## 4) Verify extracted data tests
- Files: occurrences of TODO: "Verify extracted data"
- Beschreibung: Add tests and validation logic for data extraction (export endpoints) to ensure compliance exports are complete and well-formed.
- Labels: test, compliance, low-priority
- Aufwand: 1-2 PT

## 5) Capability checking & runtime capability requests
- Files: `VERSEGUY_COMPLETE_IMPLEMENTATION_GUIDE.md` (TODOs: capability checking, runtime capability request)
- Beschreibung: Implement and document capability checks and runtime capability request/approval flow used by plugins and adapters.
- Labels: feature, design, medium-priority
- Aufwand: 2-5 PT

## 6) Developer / API Reference placeholder
- Files: `developer/API_REFERENCE.md` (placeholder TODO), `docs/index.md`
- Beschreibung: Produce an API reference (OpenAPI generation or human-written) and link it in docs index.
- Labels: docs, low-priority
- Aufwand: 1-3 PT

## 7) Misc: Backups / full-text search / replication
- Files/Refs: `VERSEGUY_MISSING_AND_IMPROVEMENTS.md` (Missing list)
- Beschreibung: These are architectural items requiring scoping; create separate epics for backup/restore automation, full-text search (e.g., Bleve/Meili), and replication strategy.
- Labels: architecture, epic
- Aufwand: variable

---

Wenn du möchtest, kann ich für jeden Eintrag gleich ein Issue-Template erzeugen (Titel + Beschreibung + repro steps + suggested labels) und sie als draft issues vorbereiten (lokale MD files oder via GitHub API wenn Token verfügbar). Sag mir kurz, ob ich die Issues anlege oder nur die MD‑Stubs erstelle.