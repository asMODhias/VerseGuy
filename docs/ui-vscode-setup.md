# Windows UI Setup for VS Code (VerseguY)

Diese Anleitung beschreibt die notwendigen Schritte, um das WinUI/Windows App SDK basierte UI lokal unter Windows in Visual Studio Code zu bauen.

## Voraussetzungen
- Windows 10/11 mit Admin-Rechten zum Installieren von SDKs/Tools.
- `dotnet` SDK 8.x (prüfen mit `dotnet --info`).
- Windows 10 SDK **10.0.22621** (erforderlich für Target `net8.0-windows10.0.22621.0`).
- Empfohlen: Visual Studio Community (oder Build Tools) mit folgenden Komponenten:
  - Universal Windows Platform development
  - Windows App SDK / WinUI Tools
  - Optional: C++ build tools (für Core native builds)
- `cmake` im PATH, wenn du C++ Core-Tests lokal bauen willst.

## Installation (empfohlen über Visual Studio)
1. Öffne **Visual Studio Installer** → **Modify** deine Installation.
2. Unter **Workloads** wähle **Universal Windows Platform development**.
3. Unter **Individual components** stelle sicher, dass **Windows 10 SDK (10.0.22621.0)** und **Windows App SDK / WinUI**-Werkzeuge ausgewählt sind.
4. Installiere die Änderungen und starte dein Terminal/VS Code neu.

## Alternative: Build Tools / SDKs ohne vollständiges Visual Studio
- Lade die **Visual Studio Build Tools** und wähle MSBuild / C++ / UWP Komponenten aus.
- Installiere separat die **Windows 10 SDK 10.0.22621** von Microsoft (Windows 10 SDK Download Center).

## Verifikation
Führe im VS Code Terminal aus:

```powershell
dotnet --info
dotnet build ui/native/VerseguY.UI/VerseguY.UI.csproj -c Release
```

- Wenn `dotnet --info` RIDs und SDKs anzeigt, ist das ein gutes Zeichen.
- `dotnet build` sollte erfolgreich durchlaufen; falls nicht, prüfe die Fehlermeldung (s.u.).

## Häufige Fehler & Troubleshooting
- Fehler: `NETSDK1083: Der angegebene RuntimeIdentifier "win10-arm" wird nicht erkannt.`
  - Ursache: Fehlende Windows SDK/RID-Definitionen oder fehlende Windows App SDK Workloads.
  - Lösung: Installiere **Windows 10 SDK 10.0.22621** und (wenn mit Visual Studio) die Windows App SDK/WinUI-Komponenten.

- Fehler: `NU1603: depends on Microsoft.WindowsAppSDK (>= 1.4.3) but ... not found.`
  - Lösung: `dotnet restore` sollte ein passendes Windows App SDK NuGet-Paket auflösen; stelle sicher, dass nuget/online Zugriff möglich ist und die vorgeschlagene Version kompatibel ist.

- Tipp: Falls du aufwändige UI-Builds vermeiden willst (z.B. auf CI-Agenten ohne WinUI-Tooling), nutze die lokale CI-Fallback-Option: `scripts/ci-local.ps1` setzt den UI-Build standardmäßig non-fatal. Setze `CI_STRICT_UI_BUILD=1`, wenn du den Build streng erzwingen willst.

## Optional: VS Code Konfiguration
- Extensions empfohlen:
  - Rust Analyzer
  - C/C++ (Microsoft)
  - CMake Tools
  - .NET Core Test Explorer (optional)
- Im Workspace: öffne das Terminal in VS Code und führe die Build- & Test-Befehle aus (siehe `scripts/README.md`).

---

Wenn nach der Installation `dotnet build` weiterhin scheitert, führe `dotnet build -v:detailed ui/native/VerseguY.UI/VerseguY.UI.csproj` aus und sende mir die Ausgabe — ich helfe bei der Analyse.