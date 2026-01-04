#pragma once

extern "C" {
    struct IPluginHost {
        void* (*get_storage_service)(void* host);
        void* (*get_network_service)(void* host);
        void* (*get_ui_service)(void* host);
        bool (*has_capability)(void* host, uint64_t cap);
        void (*log)(void* host, const char* level, const char* message);
    };
}
