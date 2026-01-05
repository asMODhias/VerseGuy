#ifndef VERSEGUY_IPLUGIN_H
#define VERSEGUY_IPLUGIN_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// ==============================================================================
// CAPABILITY FLAGS
// ==============================================================================

typedef enum {
    CAP_NONE                = 0,
    CAP_STORAGE_READ        = 1 << 0,   // Read from storage
    CAP_STORAGE_WRITE       = 1 << 1,   // Write to storage
    CAP_NETWORK_P2P         = 1 << 2,   // P2P networking
    CAP_NETWORK_MASTER      = 1 << 3,   // Master server connection
    CAP_UI_PANEL            = 1 << 4,   // Display UI panel
    CAP_UI_NOTIFICATION     = 1 << 5,   // Show notifications
    CAP_FILESYSTEM_READ     = 1 << 6,   // Read files
    CAP_FILESYSTEM_WRITE    = 1 << 7,   // Write files
    CAP_SUBPROCESS          = 1 << 8,   // Spawn subprocesses
    CAP_SYSTEM_INFO         = 1 << 9,   // Read system information
} Capability;

// ==============================================================================
// PLUGIN INTERFACE
// ==============================================================================

typedef struct IPlugin IPlugin;
typedef struct IPluginHost IPluginHost;

struct IPlugin {
    /// Get plugin unique ID (e.g., "org.verseguy.organization")
    const char* (*get_id)(void* self);
    
    /// Get plugin display name (e.g., "Organization Management")
    const char* (*get_name)(void* self);
    
    /// Get plugin version (e.g., "2.0.0")
    const char* (*get_version)(void* self);
    
    /// Get required capabilities (bitmask of Capability flags)
    uint64_t (*get_capabilities)(void* self);
    
    /// Initialize plugin with host interface
    /// Returns true on success, false on failure
    bool (*initialize)(void* self, IPluginHost* host);
    
    /// Shutdown plugin and cleanup resources
    void (*shutdown)(void* self);
    
    /// Handle event from event bus
    /// event_type: Type of event (e.g., "user_logged_in")
    /// event_data: JSON-serialized event data
    void (*on_event)(void* self, const char* event_type, const char* event_data);
    
    /// Plugin instance data (opaque pointer)
    void* instance;
};

// NOTE: The concrete `IPluginHost` struct is defined in `IPluginHost.h` to avoid duplicate definitions across headers.
// We keep only a forward-declaration here to reference the host interface.
// (See: core/include/IPluginHost.h)

// Forward declaration (detailed definition in IPluginHost.h)
// typedef struct IPluginHost IPluginHost;  -- already declared above


// ==============================================================================
// PLUGIN ENTRY POINT
// ==============================================================================

/// Plugin entry point - must be exported by plugin DLL
/// Returns pointer to IPlugin interface, or NULL on error
IPlugin* PluginInit(void);

#ifdef __cplusplus
}
#endif

#endif // VERSEGUY_IPLUGIN_H
