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
    std::string cmd_str = cmd.str();
    std::cout << "PluginLoader: executing: " << cmd_str << std::endl;
    int rc = std::system(cmd_str.c_str());
    std::cout << "PluginLoader: rc = " << rc << std::endl;
    if (rc == 0) return true;

    // Fallback to cargo run which will build & run the tool
    std::ostringstream cargo_cmd;
    cargo_cmd << "cargo run -p master_server --bin manifest-tool -- verify --manifest \"" << manifest_path << "\" --sig \"" << sig_path << "\" --pubkey \"" << pubkey_path << "\"";
    std::string cargo_cmd_str = cargo_cmd.str();
    std::cout << "PluginLoader: fallback executing: " << cargo_cmd_str << std::endl;
    rc = std::system(cargo_cmd_str.c_str());
    std::cout << "PluginLoader: fallback rc = " << rc << std::endl;
    return rc == 0;
}
