# WinUI / WebView2 Bridge (Message Handler) — Sketch

Dieses Dokument zeigt ein **minimal** sicheres Beispiel, wie die native WinUI App (C++/WinRT oder C++) WebView2‑Nachrichten empfangen und sichere Operationen (z. B. secureStorage) ausführen kann.

Wichtig: Dies ist eine **Skizze** — Review, Errorhandling, Tests und sichere Key‑Management‑Erweiterungen sind Pflicht vor Produktion.

Konzept

- Web: sendet JSON‑Nachrichten via `window.chrome.webview.postMessage({ id, type, key, value })`.
- Native: registriert `AddWebMessageReceived` Handler und parst JSON, führt geschützte Operationen aus (z. B. DPAPI auf Windows) und antwortet mit `{ id, ok: true | false, value? }`.
- Nur einfache CRUD‑Operationen (get/set/remove) für Schlüssel zulassen.

C++ Skizze (WebView2 handler)

```cpp
// NativeBridge.cpp (Skizze)
#include <webview2.h>
#include <windows.h>
#include <nlohmann/json.hpp> // assume allowed for parsing

using json = nlohmann::json;

void OnWebMessageReceived(IWebView2* webview, ICoreWebView2WebMessageReceivedEventArgs* args) {
    wil::unique_cotaskmem_string s;
    args->TryGetWebMessageAsString(&s);
    json req = json::parse(std::wstring(s.get()).begin(), std::wstring(s.get()).end());

    std::string id = req["id"].get<std::string>();
    std::string type = req["type"].get<std::string>();
    std::string key = req["key"].get<std::string>();

    json res = { {"id", id}, {"ok", false} };

    try {
      if (type == "secureStorage.set") {
        std::string value = req["value"].get<std::string>();
        // Use DPAPI to protect data
        DATA_BLOB in, out;
        in.pbData = (BYTE*)value.data();
        in.cbData = (DWORD)value.size();
        if (!CryptProtectData(&in, nullptr, nullptr, nullptr, nullptr, 0, &out)) throw std::runtime_error("dpapi failed");
        // Persist out.pbData/out.cbData to a file named by key
        SaveProtectedBlobForKey(key, out.pbData, out.cbData);
        LocalFree(out.pbData);
        res["ok"] = true;
      } else if (type == "secureStorage.get") {
        // Read protected blob, unprotect
        std::vector<BYTE> blob = ReadProtectedBlobForKey(key);
        DATA_BLOB in{ blob.size(), blob.data() };
        DATA_BLOB out;
        if (!CryptUnprotectData(&in, nullptr, nullptr, nullptr, nullptr, 0, &out)) throw std::runtime_error("dpapi unprotect failed");
        std::string value((char*)out.pbData, out.cbData);
        LocalFree(out.pbData);
        res["ok"] = true;
        res["value"] = value;
      } else if (type == "secureStorage.remove") {
        RemoveProtectedBlobForKey(key);
        res["ok"] = true;
      }
    } catch (std::exception &e) {
      res["ok"] = false;
      res["error"] = e.what();
    }

    // Send response back to webview
    std::string msg = res.dump();
    webview->PostWebMessageAsString(std::wstring(msg.begin(), msg.end()).c_str());
}
```

Integration & Tests

- Unit: native helper functions (DPAPI wrapper) mit unit tests if possible.
- E2E: start WinUI test instance on Windows runner, open WebView2 page that posts messages and asserts responses.

Sicherheitshinweis

- Schlüsselmaterial niemals in logs schreiben.
- Validiere eingehende Nachrichtentypen streng.
- Ein minimaler API‑Oberfläche (get/set/remove) reduziert Angriffsfläche.

Wenn gewünscht, kann ich eine minimal C++ Datei im Repo anlegen (nur als Beispiel) und eine GH‑Action skizze für E2E Integration hinzufügen.