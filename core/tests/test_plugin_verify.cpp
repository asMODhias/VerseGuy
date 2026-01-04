#include <iostream>
#include <fstream>
#include <string>
#include <cstdlib>
#include <filesystem>
#include "../include/PluginLoader.h"

int main() {
    namespace fs = std::filesystem;
    auto dir = fs::temp_directory_path() / "verseguy_plugin_test";
    fs::create_directories(dir);
    auto manifest = dir / "manifest.json";
    auto sig = dir / "manifest.sig";
    auto key = dir / "manifest.pub";

    // Create a manifest
    std::ofstream(manifest) << R"({"id":"org.test.plugin","name":"Test Plugin","version":"1.0.0"})";

    // Sign manifest using manifest-tool (via cargo run)
    std::string cmd = "cargo run -p master_server --bin manifest-tool -- sign \"" + manifest.string() + "\" \"" + sig.string() + "\" \"" + (dir / "kp.bin").string() + "\" \"" + key.string() + "\"";
    std::cout << "Running: " << cmd << std::endl;
    int rc = std::system(cmd.c_str());
    if (rc != 0) {
        std::cerr << "Failed to sign manifest (rc=" << rc << ")" << std::endl;
        return 1;
    }

    // Verify should succeed
    bool ok = PluginLoader::verify_manifest_with_tool(manifest.string(), sig.string(), key.string());
    std::cout << "Verification result: " << ok << std::endl;
    if (!ok) return 2;

    // Tamper manifest
    std::ofstream(manifest) << R"({"id":"org.test.plugin","name":"Test Plugin Tampered","version":"1.0.0"})";

    bool ok2 = PluginLoader::verify_manifest_with_tool(manifest.string(), sig.string(), key.string());
    std::cout << "Verification after tamper: " << ok2 << std::endl;
    if (ok2) return 3;

    std::cout << "test_plugin_verify passed" << std::endl;
    return 0;
}
