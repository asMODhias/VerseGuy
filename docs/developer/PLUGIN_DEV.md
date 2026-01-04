# Plugin Development Guide

Plugin development follows the spec: every plugin ships with a `manifest.toml`, declares capabilities and sub-plugins when applicable.

Key points:
- Use the manifest to declare `id`, `version`, required capabilities and license requirements.
- Follow the SDK expectations: implement `IPlugin`/`IPluginHost` surface for native host integration.
- Prefer WASM for untrusted plugins; native DLLs require signing and stricter review.

Manifest example and further details are in `docs/architecture/PLUGINS.md` and `plugins/` template crates.