# verseguy-audit

Simple audit event model and storage for VerseGuy.

Features:
- Record structured audit events
- Query recent events
- Storage-backed via `verseguy_storage_infra::Repository<T>`

Intended usage:
- Record important actions (user management, license changes, permission grants)
- Retention & compliance policies implemented at service layer

## Debugging des `retention_runner` 🔧

Wenn ein Lauf des `retention_runner` in CI oder lokal scheinbar "hängt", benutze den Debug-Wrapper `retention_runner_debug` um einen Timeout zu erzwingen und stdout/stderr einzufangen.

- Binary bauen:

  ```sh
  cargo build -p verseguy_audit_infra --bin retention_runner_debug
  ```

- Beispielausführung (Timeout = 30 Sekunden, `--` leitet Argumente an den `retention_runner` weiter):

  ```sh
  # gebautes Binary verwenden (schnell)
  target/debug/retention_runner_debug --timeout-seconds 30 -- --db-path ./tmp/db --days 1 --dry-run

  # oder falls nicht gebaut, nutzt der Wrapper `cargo run` (etwas langsamer):
  cargo run -p verseguy_audit_infra --bin retention_runner -- --db-path ./tmp/db --days 1 --dry-run
  ```

- Verhalten:
  - Bei Erreichen des Timeouts wird der Prozess mit Exit-Code `124` beendet und **partial** stdout/stderr werden auf die Konsole ausgegeben. Dadurch sind lange Hänger leichter zu diagnostizieren.

## CI: Smoke-Workflow

Es wurde ein GitHub Actions Workflow `.github/workflows/retention-runner-smoke.yml` hinzugefügt, der auf `ubuntu-latest`:

- erforderliche native Abhängigkeiten installiert (snappy, zlib, bz2, lz4, zstd, openssl)
- das `retention_runner`-Binary im Release-Modus baut
- einen einfachen Dry-Run ausführt und `smoke.out` / `smoke.err` sowie das Binary als Artefakte hochlädt

Artifacts findest du im Actions-Run (zugehöriger PR / Push): `retention-runner-smoke` — die Logs helfen, unerwartete Hänger oder Fehler in unterschiedlichen Umgebungen schnell zu analysieren.

---

## FAQ: Häufige Ursachen für Hänger 🧭

- Erstmalige Kompilierung / lange Build-Zeit
  - Wenn `cargo` beim ersten Mal viele Abhängigkeiten (inkl. native crates wie `rocksdb`) kompiliert, kann das wie ein "Hänger" aussehen. Abhilfe: einmal vorher bauen (`cargo build -p verseguy_audit_infra --bin retention_runner`) oder den Debug-Wrapper mit längerem Timeout verwenden.

- Datenbank-Lock (RocksDB)
  - RocksDB sperrt die DB-Dateien exklusiv; wenn ein anderer Prozess die DB offen hat, kann ein Start blockieren oder fehlschlagen. Prüfe laufende Prozesse, die die DB nutzen (Linux: `lsof <db_path>`; Windows: Process Explorer / Handle).

- Fehlende native Abhängigkeiten
  - Fehlende libs (snappy, zlib, bzip2, lz4, zstd, openssl) können Kompilationen abbrechen oder CI-Jobs zum Scheitern bringen; unser Smoke-Workflow installiert die üblichen Pakete auf `ubuntu-latest`.

- Interaktive Eingabe / falsche Flags
  - Stelle sicher, dass alle benötigten Argumente übergeben werden (z. B. `--db-path`, `--days`, `--dry-run`); interaktive Prompts blockieren Automation.

- Netzwerk / extern abhängige Services
  - Falls ein Lauf auf entfernte Dienste zugreift (z. B. S3, remote DB), können Timeouts oder langsame Antworten zu Verzögerungen führen.

- Wo finde ich Logs?
  - Lokal: nutze `retention_runner_debug` (gibt bei Timeout partial stdout/stderr aus).
  - CI: Action-Artefakte unter dem Run-Namen `retention-runner-smoke` (enthält `smoke.out`, `smoke.err` und das gebaute Binary).

### Windows: Kurze Diagnose-Tipps 🪟

- Process Explorer (Sysinternals)
  - Lade Process Explorer herunter und nutze "Find -> Find Handle or DLL..." (Ctrl+F) und suche nach dem DB-Pfad (z. B. `C:\path\to\db`) um Prozesse zu identifizieren, die die DB-Dateien geöffnet haben.

- Handle (Sysinternals)
  - Mit `handle.exe` lässt sich per Kommandozeile suchen:

    ```powershell
    handle.exe -u C:\path\to\db
    ```

  - `handle` zeigt Prozess-IDs, die Datei-Handles offen haben; diese können mit Process Explorer oder `taskkill /PID <pid>` weiter untersucht/terminiert werden.

### Linux: Automatisierte Checks im Smoke-Workflow 🐧

- Wir führen jetzt einen `lsof`-Check nach dem Smoke-Run aus, um offene Handles gegen das Smoke-DB-Verzeichnis zu erfassen. Falls `lsof` Prozesse findet, werden die Ergebnisse als `lsof.txt` hochgeladen, damit du im Actions-Artefakt genau sehen kannst, welcher Prozess die Dateien hält.

---

Wenn du möchtest, ergänze ich noch Beispielbefehle für spezifische Windows-Tools (PowerShell/Sysinternals) oder erweitere den Smoke-Job um weitere diagnostische Prüfungen (z. B. `ss`/`netstat` oder Ressourcen-Checks).
