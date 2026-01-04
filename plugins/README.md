# Plugins — Übersicht & Registry (Kurzreferenz) 🔌

Dieses Verzeichnis enthält Plugin‑Crates und eine einfache **Registry** zur Laufzeitverwaltung von Plugins.

## Struktur

- `plugins/registry` — zentrale, in‑memory Registry (mit Tests)
- `plugins/base/*` — Standardplugin‑Bibliotheken (z. B. `organization`, `fleet`, `operations`)
- `plugins/pro` — Pro‑Plugins (example)
- `plugins/adapters` — Adapter‑Plugins (example)
- `plugins/template` — Plugin‑Template (Manifest + Cargo crate + tests)

Manifests are provided for `organization`, `fleet` and `operations` under each plugin folder.
## Ziel
Die Plugin‑Architektur soll leichtgewichtig sein: jeder Plugin‑Crate ist eine normale Rust‑Bibliothek, die über den Registry‑Dienst zur Laufzeit registriert werden kann.

## Registry — schnelle Nutzung (Rust)

```rust
use plugins_registry::Registry;
use plugins_registry::PluginInfo;

let reg = Registry::new();
let p = PluginInfo { id: "example".into(), name: "Example".into(), version: "0.1".into() };
reg.register(p.clone());
let all = reg.list();
assert!(all.iter().any(|x| x.id == "example"));
let found = reg.find("example").expect("should be present");
assert_eq!(found.version, "0.1");
```

- `register` fügt ein Plugin hinzu oder ersetzt ein vorhandenes mit derselben `id`.
- `list` gibt alle registrierten Plugins zurück.
- `find(id)` sucht ein Plugin nach ID.

## Tests
- Jedes Plugin‑Crate enthält kleine Smoke‑Tests. Führe alle Tests mit:

```bash
cargo test --workspace
```

Oder nur Registry‑Tests:

```bash
cargo test -p plugins-registry
```

## Wie füge ich ein neues Plugin hinzu? ✨
1. Lege ein neues Verzeichnis `plugins/<name>` an.
2. Erzeuge ein `Cargo.toml` mit Bibliothekstyp und `src/lib.rs`.
3. Implementiere Tests unter `src`/`tests` und dokumentiere die API im Plugin‑README.
4. Füge `"plugins/<name>"` zum Workspace in `Cargo.toml` hinzu, wenn nötig.

## Hinweise
- Plugins sind derzeit einfache Rust‑Crates; die Registry ist absichtlich minimal und für POC/Evaluation gedacht.
- Bei Erweiterungen (z. B. dynamisches Laden) sollten Designentscheidungen über ABI/Symbol‑Stability und Sicherheit getroffen werden.

---

Wenn du möchtest, kann ich noch eine Beispiel‑Integration (Server bindet Registry & lädt Plugins) oder ein Template‑`plugin` Crate erstellen — sag kurz Bescheid: **yes-template** oder **nein**.