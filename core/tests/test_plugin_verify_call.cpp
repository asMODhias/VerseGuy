#include <Windows.h>
#include <iostream>
#include <fstream>
#include <filesystem>

int main() {
    namespace fs = std::filesystem;
    auto dir = fs::temp_directory_path() / "verseguy_plugin_test_call";
    fs::create_directories(dir);
    auto manifest = dir / "manifest.json";
    auto sig = dir / "manifest.sig";
    auto key = dir / "manifest.pub";
    auto kp = dir / "kp.bin";

    std::ofstream(manifest) << R"({"id":"org.test.plugin","name":"Test Plugin","version":"1.0.0"})";

    // Sign manifest using manifest-tool (cargo run)
    std::string cmd = "cargo run -p master_server --bin manifest-tool -- sign \"" + manifest.string() + "\" \"" + sig.string() + "\" \"" + kp.string() + "\" \"" + key.string() + "\"";
    std::cout << "Running: " << cmd << std::endl;
    int rc = std::system(cmd.c_str());
    if (rc != 0) {
        std::cerr << "Failed to sign manifest (rc=" << rc << ")" << std::endl;
        return 1;
    }

    HMODULE h = LoadLibraryW(L"VerseguY.Core.dll");
    if (!h) {
        std::cerr << "Failed to load core DLL" << std::endl;
        return 2;
    }

    using VerifyFn = int(*)(const char*, const char*, const char*);
    VerifyFn verify = (VerifyFn)GetProcAddress(h, "VerifyPluginManifest");
    if (!verify) {
        std::cerr << "VerifyPluginManifest not found" << std::endl;
        FreeLibrary(h);
        return 3;
    }

    int v = verify(manifest.string().c_str(), sig.string().c_str(), key.string().c_str());
    std::cout << "Verify returned: " << v << std::endl;
    if (v != 0) {
        FreeLibrary(h);
        return 4;
    }

    // tamper manifest
    std::ofstream(manifest) << R"({"id":"org.test.plugin","name":"Test Plugin Tampered","version":"1.0.0"})";
    int v2 = verify(manifest.string().c_str(), sig.string().c_str(), key.string().c_str());
    std::cout << "Verify after tamper: " << v2 << std::endl;
    if (v2 == 0) {
        FreeLibrary(h);
        return 5;
    }

    FreeLibrary(h);
    std::cout << "test_plugin_verify_call passed" << std::endl;
    return 0;
}
