# Secure Native Storage (WebView2 + Native Host)

Ziel: sichere Speicherung sensibler Daten (Zugriffstoken, Refresh‑Token, Zertifikate) ohne Geheimnisse im Web‑Bundle oder lokalen unsicheren Stores.

Kurzfassung

- Auf Windows verwenden wir die OS‑geschützten Stores (DPAPI / Credential Manager / Windows Data Protection API).
- Die WebView2 Web‑App kommuniziert mit dem nativen Host über das `window.chrome.webview.postMessage`‑Interface.
- Der native Host exponiert ein sehr kleines API (get/set/remove) über Nachrichten; die Web‑App ruft die API über eine Promise‑basierte Wrapper‑Funktion auf.
- Falls kein nativer Host verfügbar ist (z. B. beim lokalen `vite`‑Dev Server), fällt die Implementierung auf `localStorage` zurück **mit deutlichem Warnhinweis** (NICHT für Produktion empfohlen).

Host API (Nachrichten‑Protokoll)

- Request (von Web → Host):
  - { id: string, type: 'secureStorage.get' | 'secureStorage.set' | 'secureStorage.remove', key: string, value?: string }
- Response (Host → Web):
  - { id: string, ok: true, value?: string } oder { id: string, ok: false, error: string }

Sicherheitsrichtlinien

- Niemals Geheimnisse im Repo oder unverschlüsselt auf der Festplatte speichern.
- Native Host muss OS‑Schutz verwenden (DPAPI, Credential Manager, Keychain auf macOS).
- Minimale Angriffsfläche: nur `get/set/remove` für gekapselte Schlüsselpaare; keine komplexe Abfragesprache.
- Host‑Code MUSS validierte Eingaben prüfen und keine beliebigen Systemaufrufe ausführen.
- Logging: keine sensiblen Inhalte loggen.

Beispiel — JavaScript Wrapper (Web)

```ts
// ui/web/src/api/secureStorage.ts (Beispiel)
// - Versucht WebView2 Host via window.chrome.webview
// - Fallback: localStorage (WARNUNG)
```

Beispiel — Win32 Native Host (C++ Skizze)

- Implementieren Sie eine Schleife, die WebView2 Nachrichten entgegennimmt, anhand `type` entscheidet und `CryptProtectData` / `CryptUnprotectData` (Windows) verwendet, um Daten sicher zu speichern bzw. wiederherzustellen.
- Beispiel (Skizze):

```cpp
// Pseudocode (nur als Hinweis):
// on_message(msg) {
//   if (msg.type == "secureStorage.set") {
//     auto protected = CryptProtectData(msg.value);
//     StoreToFile(msg.key, protected);
//     send_response({ id: msg.id, ok: true });
//   }
//   if (msg.type == "secureStorage.get") {
//     auto protected = ReadFromFile(msg.key);
//     auto value = CryptUnprotectData(protected);
//     send_response({ id: msg.id, ok:true, value });
//   }
// }
```

CI & Testing Hinweise

- Unit tests in Web sollten den Fallback‑Pfad (localStorage) abdecken.
- E2E Tests auf Windows Runner können die native Host‑Bridge testen (Integrationstest), sofern Runner die native Komponente startet.

Dokumentation & Migration

- Kennzeichne in der Doku klar, welche Schlüssel vertraulich sind (z. B. `auth.access_token`, `auth.refresh_token`).
- Migration: beim Start prüft der Host, ob Daten im unsicheren `localStorage` liegen und bietet optional an, diese beim ersten Start in den sicheren Speicher zu migrieren.

---

Wenn du willst, kann ich jetzt eine WinUI/C++ Beispielimplementierung (minimal) für die Nachricht‑Handler skizzieren und eine einfache Integrationstest‑Anweisung für GH Actions hinzufügen.