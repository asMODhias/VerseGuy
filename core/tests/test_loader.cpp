#include <Windows.h>
#include <iostream>

int main() {
    HMODULE h = LoadLibraryW(L"VerseguY.Core.dll");
    if (!h) {
        std::cerr << "Failed to load core DLL" << std::endl;
        return 1;
    }

    using InitFunc = void (*)();
    InitFunc init = (InitFunc)GetProcAddress(h, "Initialize");
    if (!init) {
        std::cerr << "Initialize not found" << std::endl;
        return 2;
    }

    init();
    std::cout << "Core initialize invoked successfully" << std::endl;
    FreeLibrary(h);
    return 0;
}
