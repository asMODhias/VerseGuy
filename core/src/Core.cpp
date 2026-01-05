#include "IPlugin.h"
#include "IPluginHost.h"
#include "Capabilities.h"
#include "PluginLoader.h"
#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <filesystem>

#ifdef _WIN32
    #include <windows.h>
    #define EXPORT extern "C" __declspec(dllexport)
#else
    #include <dlfcn.h>
    #define EXPORT extern "C" __attribute__((visibility("default")) )
#endif

namespace fs = std::filesystem;

// Concrete host implementation
class PluginHostImpl {
public:
    PluginHostImpl() {
        host_.get_storage_service = nullptr;
        host_.get_network_service = nullptr;
        host_.get_ui_service = nullptr;
        host_.has_capability = &PluginHostImpl::has_capability_c;
        host_.log = &PluginHostImpl::log_c;
        host_.instance = this;
        capability_mask_ = UINT64_MAX; // grant all by default
    }

    bool load_plugin(const std::string& path) {
        #ifdef _WIN32
        HMODULE handle = LoadLibraryA(path.c_str());
        if (!handle) {
            log_impl("error", std::string("Failed to load plugin DLL: ") + path);
            return false;
        }
        auto init_func = (IPlugin* (*)())GetProcAddress(handle, "PluginInit");
        if (!init_func) {
            log_impl("error", std::string("Plugin missing PluginInit: ") + path);
            FreeLibrary(handle);
            return false;
        }
        #else
        void* handle = dlopen(path.c_str(), RTLD_NOW);
        if (!handle) {
            log_impl("error", std::string("Failed to load plugin: ") + std::string(dlerror()));
            return false;
        }
        auto init_func = (IPlugin* (*)())dlsym(handle, "PluginInit");
        if (!init_func) {
            log_impl("error", std::string("Plugin missing PluginInit: ") + std::string(dlerror()));
            dlclose(handle);
            return false;
        }
        #endif

        IPlugin* plugin = init_func();
        if (!plugin) {
            log_impl("error", "PluginInit returned null");
            return false;
        }

        const char* id = plugin->get_id(plugin->instance);
        const char* name = plugin->get_name(plugin->instance);
        const char* version = plugin->get_version(plugin->instance);

        log_impl("info", std::string("Loading plugin: ") + id + " v" + version);

        if (!plugin->initialize(plugin->instance, reinterpret_cast<IPluginHost*>(&host_))) {
            log_impl("error", std::string("Plugin initialization failed: ") + id);
            return false;
        }

        plugins_.push_back(plugin);
        return true;
    }

    void discover_and_load_plugins() {
        fs::path plugins_dir = fs::current_path() / "plugins";
        if (!fs::exists(plugins_dir)) {
            log_impl("warn", std::string("Plugins directory not found: ") + plugins_dir.string());
            return;
        }

        log_impl("info", std::string("Discovering plugins in: ") + plugins_dir.string());
        for (const auto& entry : fs::recursive_directory_iterator(plugins_dir)) {
            if (entry.is_regular_file()) {
                std::string ext = entry.path().extension().string();
                #ifdef _WIN32
                if (ext == ".dll") {
                #else
                if (ext == ".so") {
                #endif
                    load_plugin(entry.path().string());
                }
            }
        }
        log_impl("info", std::string("Plugin discovery complete. Loaded: ") + std::to_string(plugins_.size()));
    }

    void shutdown() {
        for (auto& plugin : plugins_) {
            if (plugin && plugin->shutdown) {
                const char* id = plugin->get_id(plugin->instance);
                log_impl("info", std::string("Shutting down plugin: ") + id);
                plugin->shutdown(plugin->instance);
            }
        }
        plugins_.clear();
    }

    IPluginHost host_{};

private:
    static bool has_capability_c(void* self, uint64_t cap) {
        PluginHostImpl* p = reinterpret_cast<PluginHostImpl*>(self);
        return (p->capability_mask_ & cap) != 0;
    }

    static void log_c(void* self, const char* level, const char* message) {
        PluginHostImpl* p = reinterpret_cast<PluginHostImpl*>(self);
        p->log_impl(level, std::string(message));
    }

    void log_impl(const char* level, const std::string& message) {
        std::cout << "[" << level << "] " << message << std::endl;
    }

    std::vector<IPlugin*> plugins_;
    uint64_t capability_mask_;
};

static std::unique_ptr<PluginHostImpl> g_host;

EXPORT void Initialize() {
    std::cout << "=== VerseguY Core ===" << std::endl;
    g_host = std::make_unique<PluginHostImpl>();
    g_host->discover_and_load_plugins();
    std::cout << "Core initialization complete." << std::endl;
}

EXPORT void Shutdown() {
    std::cout << "Shutting down core..." << std::endl;
    if (g_host) {
        g_host->shutdown();
        g_host.reset();
    }
    std::cout << "Core shutdown complete." << std::endl;
}

EXPORT bool IsFirstRun() {
    const char* appdata = std::getenv("APPDATA");
    if (!appdata) return true;
    fs::path config_dir = fs::path(appdata) / "VerseguY";
    fs::path marker = config_dir / ".initialized";
    return !fs::exists(marker);
}
