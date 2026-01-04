#pragma once
#include <stdint.h>

extern "C" {

    // Plugin capabilities
    enum Capability : uint64_t {
        CAP_STORAGE_READ = 1ull << 0,
        CAP_STORAGE_WRITE = 1ull << 1,
        CAP_NETWORK_P2P = 1ull << 2,
        CAP_NETWORK_MASTER = 1ull << 3,
        CAP_UI_PANEL = 1ull << 4,
        CAP_NOTIFICATIONS = 1ull << 5,
        CAP_SYSTEM_FILESYSTEM_READ = 1ull << 6,
        CAP_SYSTEM_FILESYSTEM_WRITE = 1ull << 7,
    };

    // Forward declaration
    struct IPluginHost;

    // Plugin interface
    struct IPlugin {
        const char* (*get_id)(void* self);
        const char* (*get_name)(void* self);
        const char* (*get_version)(void* self);
        uint64_t (*get_required_capabilities)(void* self);
        bool (*initialize)(void* self, IPluginHost* host);
        void (*shutdown)(void* self);
        void* instance;
    };

    // Plugin entry point (each plugin exports this)
    IPlugin* PluginInit();
}
