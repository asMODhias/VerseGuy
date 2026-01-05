# Verse Guy v2.0

**Star Citizen Organization & Fleet Management**

## Status

🚧 **In Active Development** 🚧

Current Phase: Core Implementation (Week 1-2)

## Architecture

- **Core:** C++ DLL (minimal bootstrap)
- **Containers:** Rust DLLs (infrastructure)
- **Plugins:** Rust DLLs (features)
- **UI:** WinUI 3 + React

## Build

```bash
# Build everything
./scripts/build.sh

# Run tests
./scripts/test.sh

# Development mode
./scripts/dev.sh
```

## Documentation

See `docs/` directory for complete documentation.

## License

MIT License - See LICENSE file

---

# VerseguY — Master Server & Tools

Kurzbeschreibung: Dieses Repository enthält die Master-Server-Implementation, Auth/Storage-Container, C++ Core, UI-Shell und Dev/CI-Skripte für das VerseguY Projekt.

## Admin CLI (verseguy-admin)

Kurze Referenz zur Admin-CLI:

- Zweck: Verwaltung des Master‑Signing‑Keys und administrative Server-Tasks.
- Binary: `verseguy-admin` (built from `master-server` crate)

Beispiele:

- List current key info:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token <ADMIN_TOKEN> key-list
```

- Rotate master key:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token <ADMIN_TOKEN> key-rotate
```

- Import keypair from file:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token <ADMIN_TOKEN> key-import --file ./master.key
```

- Import keypair from base64 string:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token <ADMIN_TOKEN> key-import --b64 "BASE64..."
```

Env vars used during local testing:

- `MASTER_KEY_FILE` — path to master key
- `MASTER_ADMIN_TOKEN` — admin token for `x-admin-token` header
- `MASTER_DB_PATH` — RocksDB path for the server

---

## CI Smoke Test for Admin CLI

A CI job `admin-cli-smoke` was added to `.github/workflows/ci.yml` that builds the server and CLI, starts a temporary server, runs CLI smoke commands (`key-list`, `key-rotate`, `key-import`) and uploads logs as artifacts.

---

**Note:** This workspace uses `resolver = "2"` in `Cargo.toml` to enable Cargo's modern dependency resolver for projects using Rust 2021+. If you see resolver-related warnings, ensure your local Cargo is up-to-date.

---

Weitere Details und Spezifikationen siehe `VERSEGUY_COPILOT_COMPLETE.md`, `docs/openapi/admin_keys.yaml` sowie **UI‑Setup und VS Code Hinweise** in `docs/ui-vscode-setup.md`. Für schnelle Script‑Hinweise schaue in `scripts/README.md`.

Weitere Dokumentation: [Documentation Index](docs/index.md) — enthält Architektur-, Legal-, Benutzer- und Entwicklerdokumentation.

Docs status: many pages created from `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`; placeholders remain for final legal text (DPA, country-specific clauses) and full API reference generation.