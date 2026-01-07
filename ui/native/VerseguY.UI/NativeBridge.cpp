// Minimal example for receiving WebView2 messages and responding
// NOTE: This is an illustrative example and not production-ready.

#include "NativeBridge.h"
#include <windows.h>
#include <dpapi.h>
#include <fstream>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

void SaveProtectedBlobForKey(const std::string& key, BYTE* data, DWORD size) {
    std::ofstream f(key + ".blob", std::ios::binary);
    f.write((char*)data, size);
}

std::vector<BYTE> ReadProtectedBlobForKey(const std::string& key) {
    std::ifstream f(key + ".blob", std::ios::binary);
    return std::vector<BYTE>((std::istreambuf_iterator<char>(f)), std::istreambuf_iterator<char>());
}

void RemoveProtectedBlobForKey(const std::string& key) {
    DeleteFileA((key + ".blob").c_str());
}

void OnWebMessageReceived(_In_ ICoreWebView2* sender, _In_ ICoreWebView2WebMessageReceivedEventArgs* args) {
    LPWSTR msgRaw = nullptr;
    args->TryGetWebMessageAsString(&msgRaw);
    if (!msgRaw) return;

    std::wstring ws(msgRaw);
    std::string jsonStr(ws.begin(), ws.end());
    json req = json::parse(jsonStr);

    std::string id = req.value("id", "");
    std::string type = req.value("type", "");
    std::string key = req.value("key", "");

    json res = { {"id", id}, {"ok", false} };

    try {
        if (type == "secureStorage.set") {
            std::string value = req.value("value", std::string());
            DATA_BLOB in{ (DWORD)value.size(), (BYTE*)value.data() };
            DATA_BLOB out;
            if (!CryptProtectData(&in, nullptr, nullptr, nullptr, nullptr, 0, &out)) throw std::runtime_error("CryptProtectData failed");
            SaveProtectedBlobForKey(key, out.pbData, out.cbData);
            LocalFree(out.pbData);
            res["ok"] = true;
        } else if (type == "secureStorage.get") {
            auto blob = ReadProtectedBlobForKey(key);
            if (blob.empty()) {
                res["ok"] = true;
                res["value"] = nullptr;
            } else {
                DATA_BLOB in{ (DWORD)blob.size(), blob.data() };
                DATA_BLOB out;
                if (!CryptUnprotectData(&in, nullptr, nullptr, nullptr, nullptr, 0, &out)) throw std::runtime_error("CryptUnprotectData failed");
                std::string value((char*)out.pbData, out.cbData);
                LocalFree(out.pbData);
                res["ok"] = true;
                res["value"] = value;
            }
        } else if (type == "secureStorage.remove") {
            RemoveProtectedBlobForKey(key);
            res["ok"] = true;
        } else if (type == "ui.openTab") {
            // Minimal dispatcher: accept `{ id: "organization" }` and acknowledge. Actual UI code in WebView will handle activation.
            std::string tabId = req.value("tab", std::string());
            // In a full implementation we would bring native window to front or adjust routing; here we simply acknowledge.
            res["ok"] = true;
            res["tab"] = tabId;
        }
    } catch (const std::exception& e) {
        res["ok"] = false;
        res["error"] = e.what();
    }

    std::string outStr = res.dump();
    std::wstring outWs(outStr.begin(), outStr.end());
    sender->PostWebMessageAsString(outWs.c_str());
}

// Exported test helper used by E2E tests. Minimal implementation that is safe to call in test environments.
extern "C" __declspec(dllexport) void SendTestMessageToWebView()
{
    OutputDebugStringA("SendTestMessageToWebView called");
    // In a full implementation we would post a message to the WebView instance here.
}
