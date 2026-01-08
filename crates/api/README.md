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

### OpenAPI & Docs UI

This crate embeds a minimal OpenAPI specification at `GET /openapi.yaml` and serves a local documentation UI at `GET /docs`.

- The documentation page now uses a lightweight, interactive local UI (`static/swagger-ui/interactive.js`) which supports basic "Try it" interactions (GET/POST/PUT), header editor (JSON), and a JSON body editor for request payloads. Responses show status, headers and a pretty-printed body (JSON if available).

- If you prefer the full official Swagger UI (recommended for richer interactivity, OpenAPI parameter rendering and the standard UX), you may vendor the `swagger-ui-dist` bundle into `crates/api/static/swagger-ui` using the helper scripts in the repository root:

```bash
# Linux / macOS
scripts/fetch-swagger-ui.sh

# Windows PowerShell
scripts/fetch-swagger-ui.ps1 -Version 4.18.3
```

After downloading, replace `static/swagger-ui/swagger-ui-bundle.js` with the downloaded `swagger-ui-bundle.min.js` and add `swagger-ui-standalone-preset.min.js` if you want the full preset feature set.

- CI: The repository includes a workflow (`.github/workflows/docs-ci.yml`) which optionally fetches the official Swagger UI dist, runs Spectral to lint `crates/api/openapi.yaml` and executes `cargo test -p verseguy-api`. The workflow treats vendored assets as optional (the docs UI still falls back to the local interactive implementation).

- UI: The interactive docs UI now shows an **Auth** panel (fetch client credentials, clear token) and renders parameter forms for endpoints that declare `parameters` or `requestBody` (form fields) in the OpenAPI spec. Use these forms to build requests (query params or form body) and send them directly from the docs page.
- Note: `swagger-ui-dist` is MIT-licensed; check the upstream license when vendoring assets.

The OpenAPI spec includes OAuth2 securitySchemes for Authorization Code and Client Credentials flows and marks `/protected` as a secured endpoint requiring the `read` scope.

### Try the Authorization Code demo locally

You can exercise the Authorization Code flow with the local docs UI. The crate includes a demo client (`client_id: demo`, `client_secret: secret`) and an interactive helper that automates the flow:

- Open `http://localhost:3000/docs` in your browser.
- Open the browser console and run:

```js
// Opens a popup, performs the auth code redirect to the callback page, exchanges the code, and sets the Authorization header
window.performAuthorizationCodeFlow({ clientId: 'demo', clientSecret: 'secret' }).then(console.log).catch(console.error);
```

- After success the interactive UI will use the returned access token for Try-it requests (Authorization: Bearer <token>), and you can call the protected endpoint `/protected`.

You can view the API documentation locally by running the service and navigating to `http://localhost:3000/docs`.
