#pragma once
#include <string>

class PluginLoader {
public:
    // Verify a manifest using the manifest-tool (Rust)
    // Returns true if verification succeeds
    static bool verify_manifest_with_tool(const std::string& manifest_path, const std::string& sig_path, const std::string& pubkey_path);
};
