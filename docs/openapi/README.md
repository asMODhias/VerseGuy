# OpenAPI Artifacts

This folder contains OpenAPI artifacts used for API documentation and client generation.

Current files:

- `admin_keys.yaml` — initial administrative API schema (verify endpoints and admin operations).

Next steps:

- Generate OpenAPI from `master-server` routes (if not available) and add HTML/Markdown artifacts.
- Add a CI job to validate OpenAPI schemas (e.g., `openapi:validate`).
