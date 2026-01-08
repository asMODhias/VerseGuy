VerseGuy API (TEIL 13)

This minimal crate scaffolds the API surface for TEIL 13: API & UI Integration.
Current endpoints (minimal):
- GET /health
- GET /metrics (placeholder)

Next steps:
- Add auth middleware using `verseguy-auth`
- Add application endpoints and OpenAPI docs
- Add integration and e2e tests

## Persistent Token Store 🔒

The API supports an optional persistent token store backed by Sled. By default the API uses an in-memory store (ephemeral) which is suitable for tests and local development.

To enable the persistent Sled-backed store, set the environment variable `VERSEGUY_API_TOKEN_STORE` to `sled` before starting the service. The store will use `data/verseguy_tokens` under the repository by default.

Example (Linux/macOS):

```bash
export VERSEGUY_API_TOKEN_STORE=sled
cargo run -p verseguy-api
```

Example (Windows PowerShell):

```powershell
$env:VERSEGUY_API_TOKEN_STORE = 'sled'
cargo run -p verseguy-api
```

Note: Tests include an integration test which verifies persistence across process restarts using a temporary Sled database.

### Redis backend (optional)

You can also use Redis as a backend for the token store. To use Redis set:

```bash
export VERSEGUY_API_TOKEN_STORE=redis
export VERSEGUY_API_TOKEN_STORE_URL=redis://127.0.0.1/
cargo run -p verseguy-api
```

If Redis is not available, tests that require Redis will be skipped gracefully.
