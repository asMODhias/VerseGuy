# UNWRAP / EXPECT Review

Stand: Automatisch erfasst mit Workspace-Suche (`unwrap(` / `expect(`) in `src/`.

## Zusammenfassung
Gefundene Vorkommen (Auszug):

- `containers/auth/src/oauth.rs`
  - `self.states.write().unwrap()` (Mutex write lock)
  - `let mut states = self.states.write().unwrap();`
  - Kontext: Producing code (handler state map)
  - Vorschlag: Replace with `.lock().map_err(|e| anyhow!("mutex poisoned: {}", e))?` or `.expect("..." )` with clear message; better: return Result from callers.

- `containers/p2p/src/network.rs`
  - `libp2p::noise::Config::new(&keypair).unwrap()`
  - Kontext: Initialization; acceptable to use `?` and return Result in constructor
  - Vorschlag: Change constructor to return `Result<..., Error>` and propagate error.

- `plugins/registry/src/lib.rs`
  - `self.plugins.lock().unwrap()` several occurrences
  - Kontext: registry operations; prefer to handle poisoning explicitly or `expect` with message.

- `plugins/base/organization/src/lib.rs`
  - `CString::new(...).unwrap()` in plugin metadata creation
  - Kontext: static plugin metadata; use `CString::new(...).expect("Invalid plugin string")` is acceptable or validate input at compile time.

- `plugins/adapters/scunpacked/src/lib.rs` (tests)
  - Many unwraps inside test helpers and test cases (e.g., `tempdir().unwrap()`, `storage.open().unwrap()`, file writes and asserts)
  - Kontext: Tests — `unwrap` acceptable but add comments or convert to `expect` with message.

- `plugins/base/operations/src/lib.rs`
  - Several `unwrap()` in functional code
  - Kontext: production logic; need review — likely replace with proper Result propagation.

## Automatische vs manuelle Änderungen
- Niedrigrisiko / automatisierbar:
  - `unwrap` in `tests/` files → replace with `expect("...")` and add short message OR leave as-is but annotate (allowed).
  - `CString::new(...).unwrap()` for static literals → convert to `expect("invalid literal")` or keep if literal is known-valid.
  - Mutex `.lock().unwrap()` → replace with `.lock().expect("mutex poisoned: <context>")` if poisoning is unrecoverable.

- Höherer Aufwand / manuell:
  - Calls to library constructors that `unwrap()` a fallible result (e.g., `noise::Config::new(...).unwrap()`) — better to change function signatures to return `Result` and propagate errors to callers; may need additional refactors and tests.

## Vorschlag für die nächsten Schritte
1. Erzeuge einen PR mit sicheren, nicht-invasive Fixes:
   - Convert `unwrap()` in `tests/` → `expect("...")` with explanatory message.
   - Add `expect` messages for mutex `lock().unwrap()` occurrences.
   - Replace trivial `.unwrap()` on static CString literals with `.expect("invalid literal")`.

2. Separat aufnehmen: Refactor-PR(s) for production fallible calls:
   - Switch constructors/initializers to return `Result`, propagate `?`.
   - Update call sites and tests accordingly.

3. Add CI check (already configured): Clippy will fail on `clippy::unwrap_used` to prevent new occurrences.

---

Wenn du willst, erstelle ich sofort PR(s) für die sicheren Fixes (1) und markiere die verbleibenden komplexen Stellen als TODOs/Issues (2). Soll ich fortfahren?