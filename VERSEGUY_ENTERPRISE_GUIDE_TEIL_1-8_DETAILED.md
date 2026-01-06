# VERSEGUY ENTERPRISE GUIDE — TEIL 1–8 (DETAILED SOP)

**Kurz:** Diese SOT (Standard Operating Procedure) beschreibt strikt ausführbare, Schritt-für-Schritt-Anweisungen für Teil 1 bis Teil 8 des VerseGuy Enterprise-Workflows. Folge den Anweisungen genau, dokumentiere Abweichungen und dokumentiere die Ergebnisse in deinem Pull Request (PR).

---

## Inhaltsverzeichnis
1. Teil 1 — Setup & Umgebung 🔧
2. Teil 2 — Linter & Formatierung ✅
3. Teil 3 — Tests & Stabilität 🧪
4. Teil 4 — Sicherheit (cargo-audit / cargo-deny) 🔒
5. Teil 5 — Dependency Convergence (libp2p) ⚙️
6. Teil 6 — Dokumentation & Beispiele 📚
7. Teil 7 — Release / CI / Deployment 🚀
8. Teil 8 — Post-Release Validation & Audit 📝

---

> Wichtig: Diese SOT ist als striktes Arbeitsprotokoll gedacht. Bei Unklarheit oder wenn ein Schritt fehlschlägt, dokumentiere die Beobachtung, stoppe und informiere das Team (Slack/Issue), bevor du weiter machst.

---

## Teil 1 — Setup & Umgebung 🔧
**Zweck:** Sicherstellen, dass die Entwicklung- und CI-Umgebung reproduzierbar ist.

**Voraussetzungen:** Windows- oder Unix-Entwicklungsmaschine, Git-Zugang, Rust-Toolchain (stable), Visual Studio / Build-Tools (falls nötig).

**Schritte:**
1. Klone das Repository (falls noch nicht vorhanden):
   - `git clone https://github.com/<org>/verseguy.git` oder arbeite im vorhandenen Workspace `d:\Data\Star Citizen\Tools\VerseGuy`.
2. Setze die Rust-Toolchain:
   - `rustup default stable`
   - `rustup update`
3. Installiere empfohlene Tools lokal:
   - `cargo install cargo-audit cargo-deny` (optional in CI nur)
   - `rustup component add clippy rustfmt`
4. Prüfe die lokale Build- und Test-Übersicht:
   - `cargo build --workspace --all-features`
   - `cargo test --workspace`

**Erwartetes Ergebnis:** Build und Tests starten ohne Blocker (Fehler notieren und eskalieren).

**Verantwortlich:** Entwickler, der die Änderung implementiert.

---

## Teil 2 — Linter & Formatierung ✅
**Zweck:** Codequalität durch Clippy/Formatter sicherstellen; strenges Verbot von `unwrap()` / `expect()` im Produktivcode.

**Schritte:**
1. Formatiere den Arbeitsbaum:
   - `cargo fmt --all`
2. Führe Clippy streng aus:
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
3. Durchsetze Policy: disallowed-methods blockieren `unwrap`/`expect` in Production crates.
   - Prüfe `clippy.toml`/`deny.toml` auf Einträge für `disallowed-methods`.
   - Beispiel-Konfiguration (prüfen/ergänzen):
     ```toml
     deny = ["disallowed_methods"]
     disallowed-methods = ["Result::unwrap","Result::expect","Option::unwrap","Option::expect"]
     ```
4. Änderungen in Tests: Verwende `verseguy_test_utils::{must, must_opt}` statt `unwrap/expect`.

**Erwartetes Ergebnis:** Clippy läuft ohne Warnings; alle `unwrap/expect` entfernt oder nur in dokumentierten Ausnahmen (Tests mit begründeter `#[allow(...)]`).

**Verifizierung:** CI Clippy-Job grünt.

---

## Teil 3 — Tests & Stabilität 🧪
**Zweck:** Tests zuverlässig und stabil machen; Vermeidung fragiler String-Matches in P2P-Tests.

**Schritte:**
1. Ziel: Alle Tests geben zuverlässige, typisierte Assertions zurück.
2. Verwende `Result`-rückgebende Tests für asynchrone/fehleranfällige Flows:
   - `#[tokio::test] async fn foo() -> Result<(), Box<dyn std::error::Error>> { ... }`
3. Ersetze `unwrap()` in Tests durch `must()` / `must_opt()`:
   - `let td = must(TempDir::new());`
4. Für P2P-Tests: Verwende strukturierte Event-Matching (typisierte Enums) statt substring-basierten Assertions.
5. Teststabilität prüfen:
   - `cargo test --workspace` lokal
   - Führe flaky tests mehrfach aus: `for i in {1..10}; do cargo test <testname> || break; done`

**Troubleshooting:**
- Falls Tests flackern, erhöhe Timeouts, stabilisiere by waiting on explicit event conditions, oder mock zeitabhängige Komponenten.

**Erwartetes Ergebnis:** Tests stabil, flakiness unter Toleranzgrenze, keine panics durch unwrap/expect.

---

## Teil 4 — Sicherheit (cargo-audit / cargo-deny) 🔒
**Zweck:** Auffinden und Beheben von Abhängigkeits-Sicherheitsproblemen.

**Schritte:**
1. Lokaler Scan:
   - `cargo audit` (installieren falls nötig mit `cargo install cargo-audit`)
   - `cargo deny check` (dev: `cargo install cargo-deny`)
2. Sammle Berichte und priorisiere:
   - Kritisch / Hoch → sofortige Untersuchung
   - Mittel / Niedrig → planmäßige Upgrades
3. Remediation-Strategien:
   - Patch/Upgrade der betroffenen crate
   - Wenn nicht möglich, Isolierung / Abschwächung (z. B. Feature-Disable, patching via `patch.crates-io` in `Cargo.toml`)
   - Dokumentiere Entscheidung im Issue/PR
4. Integration in CI:
   - CI-Job: `cargo audit` und `cargo deny` im nightly/PR-Checks
   - Bei kritischen Funden: Block PR until resolved

**Erwartetes Ergebnis:** Jede kritische Warnung ist addressed oder hat ein dokumentiertes Acceptance-Plan.

---

## Teil 5 — Dependency Convergence (libp2p) ⚙️
**Zweck:** Vermeidung von Versionskonflikten (z. B. `libp2p-mdns` 0.45.1 vs 0.46.0) und API-Fehlern.

**Schritte:**
1. Identifiziere Versionen:
   - `cargo tree -e all | rg libp2p` oder `cargo tree -i libp2p-mdns`
2. Entscheide Konvergenzstrategie:
   - Option A (empfohlen bei großen API-Änderungen): Setze workspace-wide dependency pin im Root `Cargo.toml` oder `[patch.crates-io]`.
   - Option B: Adaptiere Code, um mit neuer API zu arbeiten; commit tests und Anpassungen.
3. Implementiere Patch im Root `Cargo.toml` falls nötig:
   ```toml
   [patch.crates-io]
   libp2p-mdns = { version = "0.46", package = "libp2p-mdns" }
   ```
4. Nach Änderungen: `cargo build --workspace` und `cargo test --workspace`.

**Erwartetes Ergebnis:** Keine API/trait conflicts mehr; alle P2P-Tests laufen sauber.

**Rollback:** Rückgängig via Revert-Commit und Issue mit Analyse.

---

## Teil 6 — Dokumentation & Beispiele 📚
**Zweck:** Alle Beispielsnippets, READMEs und Guides folgen der Policy (keine `unwrap/expect`, idiomatische Fehlerbehandlung).

**Schritte:**
1. Suche in Docs: `rg "unwrap\(|expect\(" docs/ VERSEGUY_* -n`
2. Ersetze Beispiel-Code durch klare Patterns:
   - Beispiele, die Fehler haben können: zeigen `match`, `unwrap_or_else` mit erklärendem Text oder Tests, die `Result` zurückgeben.
3. Beispiel-Policy: In Tutorials können `unwrap_or_else(|| panic!("..."))` mit erklärender Anmerkung stehen; im Produktionscode: niemals.
4. Review-Schritte:
   - PR-Checklist: `docs updated`, `clippy green`, `examples compile`.

**Erwartetes Ergebnis:** Keine docs-examples, die Clippy-Verstöße oder panics promoten.

---

## Teil 7 — Release / CI / Deployment 🚀
**Zweck:** Geordneter Release und CI-Absicherung.

**Schritte:**
1. Git-Workflow:
   - Branch: `feature/enterprise-sop/<short-desc>`
   - PR: include `CHANGELOG.md` entry, `TESTS.md` updates, link zu audit issues
2. CI-Jobs sicherstellen:
   - `build`, `test`, `clippy`, `cargo-audit`, `cargo-deny` in PR checks
   - Optional: nightly job für cargo-deny policy checks
3. Merge-Policy:
   - Mindestens 2 Reviewer, 1 Security reviewer for sensitive changes
4. Release Schritte:
   - Tagging: `vX.Y.Z`
   - Publish / Deploy artifacts according to existing release playbook

**Erwartetes Ergebnis:** Release mit grünem CI und dokumentierten Audits.

---

## Teil 8 — Post-Release Validation & Audit 📝
**Zweck:** Nachkontrolle, Monitoring und Nacharbeit.

**Schritte:**
1. Nachmerge-Checks:
   - `cargo test --workspace` auf CI Runner
   - `cargo audit` erneut
2. Monitoring:
   - Beobachte Fehler/Crashreporting und Tests für 48–72h
3. Lessons Learned & Retrospektive:
   - Schreibe kurze Retro-Notiz in PR (Was lief gut, was nicht)
4. Langfristig:
   - Dokumentiere Upgrademechanismen für kritische dependencies

**Erwartetes Ergebnis:** Keine Post-Release-Critical-Alerts; action items erstellt falls nötig.

---

## Akzeptanzkriterien ✅
- PR enthält die SOT-Datei und/oder Verweise auf die Änderungen
- `cargo clippy -- -D warnings` läuft grün in CI
- `cargo test --workspace` läuft grün
- Alle kritischen `cargo audit` Findings sind mitigiert oder dokumentiert
- Alle Beispiele in `docs/` folgen der Policy (keine panics ohne Erklärung)

---

## PR-Checkliste (empfohlen) 🧾
- [ ] Branch-Name & Commit-Messages klar
- [ ] Tests: hinzugefügt / angepasst / grün
- [ ] Clippy: grün
- [ ] cargo-audit / cargo-deny: Berichte angehängt / Maßnahmen dokumentiert
- [ ] Dokumentation aktualisiert (`VERSEGUY_ENTERPRISE_GUIDE.md`, `README.md`, etc.)
- [ ] Reviewer (min 2) und Security Reviewer

---

## Kontakt & Eskalation 📞
- Bei Blockern: Erstelle ein Issue mit Label `blocker` und ping die `#security` und `#dev` Kanäle.

---

Wenn du möchtest, erstelle ich auf Basis dieser SOT direkt einen PR mit der Datei und einem vorgeschlagenen CI-Job für `cargo-audit`/`cargo-deny`.
