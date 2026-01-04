# Scripts — Quick Developer Checklist (VS Code)

Kurzcheckliste für VS Code Nutzer, um lokal die wichtigsten Builds und die UI zu bauen.

✅ Voraussetzungen (VS Code):
- Windows 10/11
- `dotnet` SDK 8.x installiert (prüfen: `dotnet --info`)
- Windows 10 SDK **10.0.22621** (erforderlich für `net8.0-windows10.0.22621.0` targets)
- Optional: Visual Studio Community / Build Tools mit **Windows App SDK / UWP**-Komponenten
- `cmake` im PATH, wenn du C++-Core bauen willst

🔧 Schnelle Prüf- und Buildbefehle (Terminal in VS Code):
- Rust checks:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test --workspace`
- Lokale CI (angepasst, toleriert fehlendes UI-Tooling):
  - `.\\	emplates\\scripts\\ci-local.ps1` (PowerShell)
- UI Build (falls Umgebung eingerichtet):
  - `dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release`

⚠️ Lokale CI Hinweis:
- Das lokale Skript `scripts/ci-local.ps1` gibt jetzt bei fehlendem UI Build eine Warnung und fährt fort. Wenn du den UI-Build streng erzwingen möchtest, setze die Umgebungsvariable `CI_STRICT_UI_BUILD=1`.

Hilfe / Troubleshooting:
- Wenn `dotnet build` den Fehler `NETSDK1083` (RuntimeIdentifier wird nicht erkannt) anzeigt, folge den Schritten in `docs/ui-vscode-setup.md`.

---

Für tiefere Anleitungen siehe `docs/ui-vscode-setup.md`.