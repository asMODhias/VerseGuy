Title: Implement Event Bus

Description
------------
(Prepared stub from docs/issues/0002-implement-event-bus.md)

Design and implement an event bus for intra-service events (audit events, retention orchestration, plugin events). Evaluate options (in-process channel + durable store vs external broker like NATS/Kafka) and provide an implementation sketch and PoC.

Files / Places
--------------
- system-wide design doc: add to `docs/architecture` or `docs/PR_DRAFTS`

Acceptance Criteria
-------------------
- Design doc with chosen approach and tradeoffs
- PoC delivering durable audit events to a subscriber
- Tests demonstrating basic pub/sub functionality

Labels: architecture, design
Estimate: 3-7 PT

---
(Prepared locally; run `scripts/create_issues_from_md.sh` with `gh` CLI to open on GitHub)