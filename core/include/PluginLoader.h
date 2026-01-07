#pragma once
#include <string>

// Export macro for Windows DLL
#if defined(_WIN32) || defined(_WIN64)
  #ifdef VERSEGUY_CORE_EXPORTS
    #define VY_API __declspec(dllexport)
  #else
    #define VY_API __declspec(dllimport)
  #endif
#else
  #define VY_API
#endif

class VY_API PluginLoader {
public:
    // Verify a manifest using the manifest-tool (Rust)
    // Returns true if verification succeeds
    static bool verify_manifest_with_tool(const std::string& manifest_path, const std::string& sig_path, const std::string& pubkey_path);
};
