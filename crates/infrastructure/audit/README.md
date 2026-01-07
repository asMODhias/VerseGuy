# verseguy-audit

Simple audit event model and storage for VerseGuy.

Features:
- Record structured audit events
- Query recent events
- Storage-backed via `verseguy_storage_infra::Repository<T>`

Intended usage:
- Record important actions (user management, license changes, permission grants)
- Retention & compliance policies implemented at service layer
