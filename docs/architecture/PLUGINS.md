# Plugins — Manifest & SDK

Verse Guy plugins implement features and may contain sub-plugins for fine-grained capabilities.

Key points:

- Plugins are signed and verified before load (code signing required for native DLLs).

- Prefer WASM for sandboxed plugins when feasible.

- Each plugin ships a `manifest.toml` describing id, version, required capabilities and sub-plugins.

Example manifest snippet (from spec):

```toml
[plugin]
id = "org.verseguy.organization"
name = "Organization Management"
version = "2.0.0"
license_required = "Free"

[capabilities]
required = ["storage:read","storage:write","ui:panel","network:p2p"]
```

Plugin SDK:

- C++ host exposes `IPlugin` and `IPluginHost` C-style interfaces.

- Rust plugins implement the surface expected by the host (see `core/include/IPlugin.h` in the spec).

Testing and security:

- All plugin entry points must validate inputs and check capabilities.

- Plugins must include unit tests and an integration test demonstrating host interaction.
