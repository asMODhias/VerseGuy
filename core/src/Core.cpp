#include "IPluginHost.h"
#include "Capabilities.h"
#include "PluginLoader.h"
#include <windows.h>
#include <iostream>

extern "C" __declspec(dllexport) void Initialize() {
    // Minimal initialization: logging to stdout (expand later)
    std::cout << "VerseGuy.Core: Initialize called" << std::endl;
}

extern "C" __declspec(dllexport) int VerifyPluginManifest(const char* manifest_path, const char* sig_path, const char* pubkey_path) {
    try {
        std::cout << "VerifyPluginManifest called with manifest=" << manifest_path << " sig=" << sig_path << " pub=" << pubkey_path << std::endl;
        bool ok = PluginLoader::verify_manifest_with_tool(manifest_path, sig_path, pubkey_path);
        std::cout << "VerifyPluginManifest result: " << (ok ? "OK" : "FAIL") << std::endl;
        return ok ? 0 : 1;
    } catch (...) {
        std::cerr << "VerifyPluginManifest: exception" << std::endl;
        return 2;
    }
}

BOOL WINAPI DllMain(HINSTANCE hinstDLL, DWORD fdwReason, LPVOID lpvReserved) {
    switch (fdwReason) {
        case DLL_PROCESS_ATTACH:
            // Optional: perform early init
            break;
        case DLL_THREAD_ATTACH:
        case DLL_THREAD_DETACH:
        case DLL_PROCESS_DETACH:
            break;
    }
    return TRUE;
}
