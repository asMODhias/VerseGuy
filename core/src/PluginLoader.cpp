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
    // Try the binary first (positional args expected)
    cmd << manifest_tool << " verify \"" << manifest_path << "\" \"" << sig_path << "\" \"" << pubkey_path << "\"";
    int rc = std::system(cmd.str().c_str());
    if (rc == 0) return true;

    // Fallback to cargo run which will build & run the tool (use positional args)
    std::ostringstream cargo_cmd;
    // Use --manifest-path so cargo finds the workspace root even when invoked from build dirs
    cargo_cmd << "cargo run --manifest-path ../../Cargo.toml -p master_server --bin manifest-tool -- verify \"" << manifest_path << "\" \"" << sig_path << "\" \"" << pubkey_path << "\"";
    rc = std::system(cargo_cmd.str().c_str());
    return rc == 0;
}
