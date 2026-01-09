# Fleet (Ships) API

Kurzbeschreibung der neuen Endpunkte (TEIL 14 — Fleet):

- POST /ships
  - Body: { "owner_id": string, "model": string, "manufacturer": string }
  - Response: 201 Created with created `Ship` JSON
- GET /ships/{owner_id}
  - Response: 200 OK with list of ships for the owner
- GET /ships/{owner_id}/{ship_id}
  - Response: 200 OK with single `Ship` JSON or 404 if not found
- PUT /ships/{owner_id}/{ship_id}
  - Body: full `Ship` JSON (owner/id must match path)
  - Response: 200 OK with updated `Ship` JSON
- DELETE /ships/{owner_id}/{ship_id}
  - Response: 204 No Content on success

Test-Hinweise

- Integrationstest: `crates/api/tests/ships.rs` (test `ships_end_to_end`) — verwendet `build_app_with_services` zum Injizieren eines temporären FleetService.
- Test-Setup:
  - Verwende `tempfile` (dev-dependency) für temporäre Datenspeicher während Tests.
  - Beim Lesen von Antwort-Bodies in Tests benutze `axum::body::to_bytes(resp.into_body(), 1024 * 1024)` (Axum benötigt Limit-Parameter für `to_bytes`).
  - `Router::oneshot()` konsumiert den Router — in Tests `app.clone().oneshot(req)` verwenden, wenn mehrere Requests im selben Test ausgeführt werden.
  - Einige Tests erlauben lokal `unwrap()`/`expect()` mit einer scoped `#[allow(clippy::disallowed_methods)]`-Annotation, um die Projekt-Lint-Policy zu respektieren.

Sonstiges

- OpenAPI: Die eingebettete OpenAPI/YAML (`/openapi.yaml`) und die UI (`/docs`) können in Folge-PRs um die neuen Endpunkte ergänzt werden.
- CI/Hinweis: Plattform-spezifische Tests (z. B. Keyring) werden in Headless/CI-Umgebungen übersprungen, um Flakiness zu vermeiden; Tests der Fleet-Endpunkte sind so konzipiert, dass sie ohne native Keyring-Abhängigkeiten laufen.
