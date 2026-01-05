#include "PluginLoader.h"
#include <cstdlib>
#include <iostream>
#include <sstream>

bool PluginLoader::verify_manifest_with_tool(const std::string& manifest_path, const std::string& sig_path, const std::string& pubkey_path) {
    // Prefer using a built binary if available
    std::string manifest_tool = "./target/release/manifest-tool";
#ifdef _WIN32
    manifest_tool = "..\\master-server\\target\\release\\manifest-tool.exe";
#endif
    std::ostringstream cmd;
    // Try the binary first
    cmd << manifest_tool << " verify --manifest \"" << manifest_path << "\" --sig \"" << sig_path << "\" --pubkey \"" << pubkey_path << "\"";
    int rc = std::system(cmd.str().c_str());
    if (rc == 0) return true;

    // Fallback to cargo run which will build & run the tool
    std::ostringstream cargo_cmd;
    // Use --manifest-path so cargo finds the workspace root even when invoked from build dirs
    cargo_cmd << "cargo run --manifest-path ../../Cargo.toml -p master_server --bin manifest-tool -- verify --manifest \"" << manifest_path << "\" --sig \"" << sig_path << "\" --pubkey \"" << pubkey_path << "\"";
    rc = std::system(cargo_cmd.str().c_str());
    return rc == 0;
}
