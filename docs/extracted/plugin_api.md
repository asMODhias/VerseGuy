# Plugin API & Manifest — Extract (Canonical)

## C++ `IPlugin` Interface (Key Methods)

```cpp
struct IPlugin {
    const char* (*get_id)(void*);
    const char* (*get_name)(void*);
    const char* (*get_version)(void*);
    uint64_t (*get_required_capabilities)(void*);
    bool (*initialize)(void*, void* host);
    void (*shutdown)(void*);
};

IPlugin* PluginInit();
```

## Plugin Host Interface

```cpp
struct IPluginHost {
    void* (*get_storage_service)(void*);
    void* (*get_network_service)(void*);
    bool (*has_capability)(void*, uint64_t cap);
    void (*log)(void*, const char* level, const char* message);
};
```

## Manifest (TOML) Example

```toml
[plugin]
id = "org.verseguy.organization"
name = "Organization Management"
version = "2.0.0"
license_required = "Free"
core_version_min = "2.0.0"

[capabilities]
required = ["storage:read","storage:write","ui:panel","network:p2p"]

[subplugins.members]
name = "Member Management"
enabled_by_default = true
license_required = "Free"
```

## Capabilities

- storage:read, storage:write
- network:p2p, network:master_server
- ui:panel, notifications

Plugins must declare capabilities; user approves at install; core enforces at runtime.

---
Source: `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`
