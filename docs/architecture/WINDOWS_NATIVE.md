# Windows Native Architecture

Verse Guy targets Windows as its primary platform and uses a hybrid DLL architecture for performance and modularity.

Highlights:
- Launcher: tiny stub executable (VerseguY.exe) that loads `VerseguY.Core.dll`.
- Core: native C++ DLL (~2MB) responsible for bootstrap, plugin loader, event bus and Windows integration.
- Containers: Rust DLLs implementing infrastructure (Auth, Storage, P2P, Compliance, Licensing).
- Plugins: Rust DLLs or WASM modules implementing features and sub-plugins.

Performance goals:
- Core startup < 500ms
- Core size < 5MB
- Minimize memory overhead and plugin load times

Security:
- Binary signing and signature verification on load.
- Capability-based permission enforcement for plugins.

See `README.md` and `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md` for full details.