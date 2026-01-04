#include <windows.h>
#include <iostream>

int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE, PWSTR, int) {
    std::wstring dllName = L"VerseguY.Core.dll";
    std::wcout << L"Launcher starting, loading " << dllName << std::endl;

    // Detect headless mode via --no-gui flag or VERSEGUY_HEADLESS env var
    bool headless = false;
    LPWSTR cmdLine = GetCommandLineW();
    if (cmdLine && wcsstr(cmdLine, L"--no-gui") != nullptr) {
        headless = true;
    } else {
        wchar_t envBuf[8] = {0};
        DWORD len = GetEnvironmentVariableW(L"VERSEGUY_HEADLESS", envBuf, (DWORD)std::size(envBuf));
        if (len > 0) { headless = true; }
    }

    HMODULE hCore = LoadLibraryW(dllName.c_str());
    if (!hCore) {
        std::wcerr << L"Failed to load " << dllName << std::endl;
        return 1;
    }

    using InitFunc = void (*)();
    auto initFunc = (InitFunc)GetProcAddress(hCore, "Initialize");
    if (!initFunc) {
        std::cerr << "Initialize not found in core DLL" << std::endl;
        FreeLibrary(hCore);
        return 2;
    }

    initFunc(); // initialize core

    if (!headless) {
        // Keep launcher alive until user closes
        MessageBoxW(NULL, L"VerseguY Launcher started. Close to exit.", L"VerseguY", MB_OK);
    } else {
        std::wcout << L"Launcher started in headless mode; exiting after Init." << std::endl;
    }

    FreeLibrary(hCore);
    return 0;
}
