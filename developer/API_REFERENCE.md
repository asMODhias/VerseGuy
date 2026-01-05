# API Reference (TODO)

This document will contain the generated API reference for the Master Server and any other HTTP APIs.

Goal
- Provide an up-to-date OpenAPI / Swagger spec and generated HTML docs for developers and integration testing.

Suggested workflow
1. Add a CI workflow to run `openapi:validate` and fail the build when the OpenAPI spec is invalid.
2. Generate docs and publish artifacts as part of the `web-build` / `docs-validate` jobs.

Local generation
- As a starting point, the OpenAPI spec at `openapi/admin_keys.yaml` should be validated.
- Example (manual):
  - `npx @apidevtools/swagger-cli validate openapi/admin_keys.yaml`

Action items
- [ ] Add `openapi:validate` CI workflow (manual trigger until validated).
- [ ] Add generated API docs to `docs/` or publish in CI artifacts.
- [ ] Add a script to automate spec generation if new endpoints are added.

If you want, I can add a skeleton `openapi-validate` workflow to `.github/workflows/` that can be triggered manually. (Recommended.)
