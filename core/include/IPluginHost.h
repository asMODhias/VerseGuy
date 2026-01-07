#pragma once
#include <stdint.h>

extern "C" {
    struct IPluginHost {
        void* (*get_storage_service)(void* host);
        void* (*get_network_service)(void* host);
        void* (*get_ui_service)(void* host);
        bool (*has_capability)(void* instance, uint64_t cap);
        void (*log)(void* instance, const char* level, const char* message);
        // Opaque instance pointer back to host implementation
        void* instance;
    };
}
