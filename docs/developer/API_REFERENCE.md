# API Reference — Overview & Generation

This page explains how API docs are produced and where to find the authoritative OpenAPI artifacts.

## Current artifacts
- `docs/openapi/admin_keys.yaml` — minimal admin-related schema.

## Recommended generation workflow
1. Annotate `master-server` routes with OpenAPI metadata (e.g., using `utoipa` or manual YAML snippets).
2. Generate OpenAPI YAML into `docs/openapi/` as part of a `cargo` task or a small generator binary (e.g., `cargo run --bin openapi-gen`).
3. Commit the generated YAML/JSON artifacts to `docs/openapi/` and produce a rendered HTML or Markdown view as part of CI.

## Important endpoints (reference from spec)
- Authentication: `POST /auth/register`, `POST /auth/login`, `POST /auth/refresh`, `GET /auth/validate`
- Verification & Revocation: `POST /verify/plugin`, `POST /verify/revoke`, `GET /verify/revocations`, `GET /verify/status/:plugin_id`
- Plugin Registry: `GET /plugins/search`, `GET /plugins/:id`, `POST /plugins/publish`
- P2P bootstrap: `GET /p2p/bootstrap/peers`, `POST /p2p/bootstrap/announce`

> Note: These endpoints are described in the specification; OpenAPI artifacts are the source-of-truth for client generation.

## CI & validation
- Add an `openapi:validate` step in CI to run schema validation and to generate client stubs if desired.

## Next steps for implementers
- Decide on a concrete OpenAPI generation approach (derive from route attributes, or maintain hand-written YAML).
- Add an internal script or crate to produce `docs/openapi/*.yaml` as part of `scripts/ci-local.ps1` or `scripts/build.sh`.
