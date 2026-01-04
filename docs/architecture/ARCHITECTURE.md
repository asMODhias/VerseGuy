# Architecture — Verse Guy V2.0

This document is a concise, authoritative extract of the Verse Guy V2.0 specification (see `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`). It summarizes the core architecture, design principles and layer responsibilities.

## Core Design Principles

- Minimal Core: The native C++ core provides bootstrap, plugin loader and UI shell only — no business logic lives in core.
- Everything is a Module: Containers provide infrastructure (Auth, Storage, Licensing, Compliance, P2P, Audit). Plugins implement features.
- Offline-First: Features work offline; the Master Server coordinates but is not mandatory for runtime.
- Windows Native Optimized: C++ core + WinUI3 + Rust containers/plugins for performance and integration.
- Security & Compliance: Capability-based permissions, signed binaries, audited append-only logs and built-in GDPR/CCPA/DSA compliance.

## Layered Architecture

- Launcher (tiny stub) → Core (C++ native DLL) → Containers (Rust DLLs) → Plugins (Rust DLLs or WASM) → UI (WinUI3 + WebView2).

See related files:

- `docs/architecture/CONTAINERS.md`
- `docs/architecture/PLUGINS.md`
- `docs/architecture/WINDOWS_NATIVE.md`

---

© Verse Guy — Specification extract
