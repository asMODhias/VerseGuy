#include <iostream>
#include <vector>
#include <string>
#include <memory>
#include <filesystem>
#include <unordered_map>
#include <mutex>
#include "../include/IPlugin.h"

#ifdef _WIN32
    #include <windows.h>
    #define EXPORT __declspec(dllexport)
    #define PATH_SEPARATOR "\\"
#else
    #include <dlfcn.h>
    #define EXPORT __attribute__((visibility("default")))
    #define PATH_SEPARATOR "/"
#endif

namespace fs = std::filesystem;

// ==============================================================================
// LOGGER
// ==============================================================================

class Logger {
public:
    static void log(const std::string& level, const std::string& message) {
        std::lock_guard<std::mutex> lock(mutex_);
        
        // Get timestamp
        auto now = std::chrono::system_clock::now();
        auto time = std::chrono::system_clock::to_time_t(now);
        
        // Format: [YYYY-MM-DD HH:MM:SS] [LEVEL] message
        char timestamp[64];
        std::strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", std::localtime(&time));
        
        std::cout << "[" << timestamp << "] [" << level << "] " << message << std::endl;
    }
    
    static void error(const std::string& msg) { log("ERROR", msg); }
    static void warn(const std::string& msg)  { log("WARN", msg); }
    static void info(const std::string& msg)  { log("INFO", msg); }
    static void debug(const std::string& msg) { log("DEBUG", msg); }
    
private:
    static std::mutex mutex_;
};

std::mutex Logger::mutex_;

// ==============================================================================
// PLUGIN MANAGER
// ==============================================================================

struct LoadedPlugin {
    std::string id;
    std::string name;
    std::string version;
    IPlugin* interface;
    void* dll_handle;
    uint64_t capabilities;
};

class PluginManager {
public:
    PluginManager() {
        Logger::info("PluginManager initialized");
    }
    
    ~PluginManager() {
        shutdown_all();
    }
    
    bool load_plugin(const fs::path& path) {
        Logger::info("Loading plugin: " + path.string());
        
        try {
            // Load DLL
            void* handle = nullptr;
            
            #ifdef _WIN32
                handle = LoadLibraryA(path.string().c_str());
                if (!handle) {
                    DWORD error = GetLastError();
                    Logger::error("Failed to load DLL (error " + std::to_string(error) + "): " + path.string());
                    return false;
                }
                
                auto init_func = (IPlugin* (*)())GetProcAddress((HMODULE)handle, "PluginInit");
                if (!init_func) {
                    Logger::error("Plugin missing PluginInit export: " + path.string());
                    FreeLibrary((HMODULE)handle);
                    return false;
                }
            #else
                handle = dlopen(path.string().c_str(), RTLD_NOW);
                if (!handle) {
                    Logger::error("Failed to load plugin: " + std::string(dlerror()));
                    return false;
                }
                
                auto init_func = (IPlugin* (*)())dlsym(handle, "PluginInit");
                if (!init_func) {
                    Logger::error("Plugin missing PluginInit: " + std::string(dlerror()));
                    dlclose(handle);
                    return false;
                }
            #endif
            
            // Call PluginInit
            IPlugin* plugin = init_func();
            if (!plugin) {
                Logger::error("PluginInit returned NULL");
                #ifdef _WIN32
                    FreeLibrary((HMODULE)handle);
                #else
                    dlclose(handle);
                #endif
                return false;
            }
            
            // Get plugin info
            const char* id = plugin->get_id(plugin->instance);
            const char* name = plugin->get_name(plugin->instance);
            const char* version = plugin->get_version(plugin->instance);
            uint64_t caps = plugin->get_capabilities(plugin->instance);
            
            if (!id || !name || !version) {
                Logger::error("Plugin returned invalid metadata");
                #ifdef _WIN32
                    FreeLibrary((HMODULE)handle);
                #else
                    dlclose(handle);
                #endif
                return false;
            }
            
            // Check for duplicate ID
            if (plugins_.find(id) != plugins_.end()) {
                Logger::error("Plugin with ID already loaded: " + std::string(id));
                #ifdef _WIN32
                    FreeLibrary((HMODULE)handle);
                #else
                    dlclose(handle);
                #endif
                return false;
            }
            
            Logger::info("Initializing plugin: " + std::string(name) + " v" + std::string(version));
            
            // Initialize plugin (will be done by caller after registering host)
            
            // Store plugin
            LoadedPlugin loaded;
            loaded.id = id;
            loaded.name = name;
            loaded.version = version;
            loaded.interface = plugin;
            loaded.dll_handle = handle;
            loaded.capabilities = caps;
            
            plugins_[id] = loaded;
            
            Logger::info("Plugin loaded successfully: " + std::string(name));
            
            return true;
            
        } catch (const std::exception& e) {
            Logger::error("Exception loading plugin: " + std::string(e.what()));
            return false;
        }
    }
    
    bool initialize_plugin(const std::string& id, IPluginHost* host) {
        auto it = plugins_.find(id);
        if (it == plugins_.end()) {
            Logger::error("Plugin not found: " + id);
            return false;
        }
        
        Logger::info("Initializing plugin: " + it->second.name);
        
        if (!it->second.interface->initialize(it->second.interface->instance, host)) {
            Logger::error("Plugin initialization failed: " + id);
            return false;
        }
        
        Logger::info("Plugin initialized: " + it->second.name);
        return true;
    }
    
    void discover_plugins(const fs::path& directory) {
        if (!fs::exists(directory)) {
            Logger::warn("Plugins directory not found: " + directory.string());
            return;
        }
        
        Logger::info("Discovering plugins in: " + directory.string());
        
        size_t count = 0;
        
        for (const auto& entry : fs::recursive_directory_iterator(directory)) {
            if (!entry.is_regular_file()) continue;
            
            std::string ext = entry.path().extension().string();
            
            #ifdef _WIN32
                if (ext == ".dll") {
            #else
                if (ext == ".so") {
            #endif
                if (load_plugin(entry.path())) {
                    count++;
                }
            }
        }
        
        Logger::info("Plugin discovery complete. Found: " + std::to_string(count));
    }
    
    void shutdown_all() {
        Logger::info("Shutting down all plugins...");
        
        for (auto& pair : plugins_) {
            LoadedPlugin& plugin = pair.second;
            
            Logger::info("Shutting down: " + plugin.name);
            
            if (plugin.interface && plugin.interface->shutdown) {
                plugin.interface->shutdown(plugin.interface->instance);
            }
            
            // Unload DLL
            #ifdef _WIN32
                FreeLibrary((HMODULE)plugin.dll_handle);
            #else
                dlclose(plugin.dll_handle);
            #endif
        }
        
        plugins_.clear();
        
        Logger::info("All plugins shut down");
    }
    
    const std::unordered_map<std::string, LoadedPlugin>& get_plugins() const {
        return plugins_;
    }
    
private:
    std::unordered_map<std::string, LoadedPlugin> plugins_;
};

// ==============================================================================
// PLUGIN HOST IMPLEMENTATION
// ==============================================================================

class PluginHost {
public:
    PluginHost(PluginManager* manager) : manager_(manager) {
        Logger::info("PluginHost initialized");
        
        // Setup interface
        interface_.get_storage = nullptr;  // Will be set by storage container
        interface_.get_network = nullptr;  // Will be set by network container
        interface_.get_ui = nullptr;       // Will be set by UI
        interface_.has_capability = PluginHost::has_capability_impl;
        interface_.log = PluginHost::log_impl;
        interface_.emit_event = PluginHost::emit_event_impl;
        interface_.request_capability = PluginHost::request_capability_impl;
        interface_.instance = this;
    }
    
    IPluginHost* get_interface() {
        return &interface_;
    }
    
private:
    static bool has_capability_impl(void* self, uint64_t cap) {
        // TODO: Implement proper capability checking
        // For now, grant all capabilities
        return true;
    }
    
    static void log_impl(void* self, const char* level, const char* message) {
        if (!level || !message) return;
        Logger::log(level, message);
    }
    
    static void emit_event_impl(void* self, const char* event_type, const char* event_data) {
        if (!event_type || !event_data) return;
        
        PluginHost* host = static_cast<PluginHost*>(self);
        Logger::debug("Event emitted: " + std::string(event_type));
        
        // TODO: Implement event bus
        // For now, just log
    }
    
    static bool request_capability_impl(void* self, uint64_t cap) {
        // TODO: Implement runtime capability request
        // For now, grant all requests
        return true;
    }
    
    PluginManager* manager_;
    IPluginHost interface_;
};

// ==============================================================================
// GLOBAL STATE
// ==============================================================================

static std::unique_ptr<PluginManager> g_plugin_manager;
static std::unique_ptr<PluginHost> g_plugin_host;

// ==============================================================================
// EXPORTED FUNCTIONS
// ==============================================================================

extern "C" {
    EXPORT void Initialize() {
        Logger::info("=================================");
        Logger::info("  VerseguY Core v2.0");
        Logger::info("=================================");
        Logger::info("Initializing core...");
        
        // Create plugin manager
        g_plugin_manager = std::make_unique<PluginManager>();
        
        // Create plugin host
        g_plugin_host = std::make_unique<PluginHost>(g_plugin_manager.get());
        
        // Discover plugins
        fs::path plugins_dir = fs::current_path() / "plugins";
        g_plugin_manager->discover_plugins(plugins_dir);
        
        // Initialize plugins
        for (const auto& pair : g_plugin_manager->get_plugins()) {
            g_plugin_manager->initialize_plugin(pair.first, g_plugin_host->get_interface());
        }
        
        Logger::info("Core initialization complete");
        Logger::info("=================================");
    }
    
    EXPORT void Shutdown() {
        Logger::info("=================================");
        Logger::info("Shutting down core...");
        
        g_plugin_manager.reset();
        g_plugin_host.reset();
        
        Logger::info("Core shutdown complete");
        Logger::info("=================================");
    }

    EXPORT bool IsFirstRun() {
        // Check for initialization marker
        #ifdef _WIN32
            const char* appdata = std::getenv("APPDATA");
            if (!appdata) return true;
            fs::path config_dir = fs::path(appdata) / "VerseguY";
        #else
            const char* home = std::getenv("HOME");
            if (!home) return true;
            fs::path config_dir = fs::path(home) / ".config" / "verseguy";
        #endif
        
        fs::path marker = config_dir / ".initialized";
        bool first_run = !fs::exists(marker);
        
        if (first_run) {
            Logger::info("First run detected");
        }
        
        return first_run;
    }
    
    EXPORT void MarkInitialized() {
        #ifdef _WIN32
            const char* appdata = std::getenv("APPDATA");
            if (!appdata) return;
            fs::path config_dir = fs::path(appdata) / "VerseguY";
        #else
            const char* home = std::getenv("HOME");
            if (!home) return;
            fs::path config_dir = fs::path(home) / ".config" / "verseguy";
        #endif
        
        // Create directory
        std::error_code ec;
        fs::create_directories(config_dir, ec);
        
        // Create marker file
        fs::path marker = config_dir / ".initialized";
        std::ofstream file(marker);
        file.close();
        
        Logger::info("Marked as initialized");
    }
}
