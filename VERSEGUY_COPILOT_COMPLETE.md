---
title: VERSEGUY COPILOT COMPLETE INSTRUCTION
version: 2.0.0-local
status: SINGLE_SOURCE_OF_TRUTH
mode: LOCAL_DEVELOPMENT_ONLY
audience: GitHub Copilot + VS Code
date: 2026-01-03
---

# ⚠️ COPILOT: READ THIS FIRST — CRITICAL RULES

## ABSOLUTE PROHIBITIONS

```yaml
NEVER_EVER:
  ❌ Create mock data
  ❌ Create stub functions  
  ❌ Use placeholder implementations
  ❌ Generate fake API responses
  ❌ Simulate database operations
  ❌ Mock authentication
  ❌ Invent architecture not defined here
  ❌ Add features not specified
  ❌ Skip error handling
  ❌ Skip logging
  ❌ Skip tests

ALWAYS_ALWAYS:
  ✅ Implement complete, working code
  ✅ Use real database (RocksDB)
  ✅ Use real networking (libp2p)
  ✅ Use real authentication (Argon2 + JWT)
  ✅ Handle all errors explicitly
  ✅ Log all important operations
  ✅ Write tests for every function
  ✅ Follow this document exactly
  ✅ Ask if anything is unclear

This_Is_A_Real_Production_Application:
  - Not a prototype
  - Not a proof-of-concept
  - Not a demo
  - REAL ENTERPRISE SOFTWARE

Environment:
  - LOCAL DEVELOPMENT ONLY
  - NO cloud dependencies
  - NO external repositories
  - NO GitHub, NO cloud storage
  - Everything runs on Windows 11
```

---

# 📦 COMPLETE PROJECT SPECIFICATION

**SEE DETAILED DOCUMENTS:**
1. `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md` - Complete architecture
2. `VERSE_GUY_V2_WINDOWS_NATIVE_OPTIMIZATION.md` - Windows DLL architecture
3. This file - Copilot-specific instructions

---

# 🏗️ ARCHITECTURE SUMMARY

```yaml
Technology_Stack:
  Core: C++20 (Native DLL, ~2MB)
  Containers: Rust 1.75+ (Infrastructure DLLs)
  Plugins: Rust 1.75+ (Feature DLLs)
  UI_Shell: WinUI 3 (C# .NET 8)
  UI_Web: React 18 + TypeScript 5
  Database: RocksDB (local)
  Network: libp2p (P2P mesh)
  Auth: Argon2 + JWT

File_Types:
  .cpp/.h: Core DLL implementation
  .rs: Containers and plugins
  .xaml/.cs: WinUI 3 UI shell
  .tsx/.ts: React web UI

Build_Tools:
  Core: CMake 3.25+
  Tests: CTest (C++ tests discovered & run via CTest; CI runs `ctest`)
  Rust: Cargo
  UI: dotnet + npm

Development:
  Environment: Windows 11 + WSL2
  VCS: Git (LOCAL ONLY)
  CI: Local GitHub Actions-style
```

---

# 📁 EXACT DIRECTORY STRUCTURE

```bash
verse-guy-v2/
├── core/                    # C++ Core DLL
│   ├── include/
│   │   ├── IPlugin.h
│   │   ├── IPluginHost.h
│   │   └── Capabilities.h
│   ├── src/
│   │   ├── main.cpp
│   │   ├── PluginLoader.cpp
│   │   └── EventBus.cpp
│   ├── tests/
│   └── CMakeLists.txt
│
├── launcher/                # Tiny stub EXE
│   ├── src/main.cpp
│   └── CMakeLists.txt
│
├── containers/              # Rust DLLs
│   ├── auth/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── local.rs
│   │   │   ├── oauth.rs
│   │   │   ├── session.rs
│   │   │   └── types.rs
│   │   ├── tests/
│   │   └── Cargo.toml
│   │
│   ├── storage/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── rocksdb.rs
│   │   │   └── schema.rs
│   │   └── Cargo.toml
│   │
│   ├── licensing/
│   ├── compliance/
│   ├── p2p/
│   └── audit/
│
├── plugins/                 # Rust Plugin DLLs
│   ├── registry/
│   ├── base/
│   │   ├── organization/
│   │   ├── fleet/
│   │   └── operations/
│   ├── pro/
│   └── adapters/
│
├── ui/
│   ├── native/              # WinUI 3
│   │   ├── MainWindow.xaml
│   │   └── VerseguY.UI.csproj
│   └── web/                 # React
│       ├── src/
│       ├── package.json
│       └── tsconfig.json
│
├── scripts/
│   ├── build.sh
│   ├── test.sh
│   └── dev.sh
│
├── Cargo.toml               # Workspace root
└── README.md
```

---

# 🔑 KEY DATA TYPES

## Authentication

```rust
// containers/auth/src/types.rs

pub enum AuthMethod {
    Local { username: String, password_hash: String },
    OAuth { provider: OAuthProvider, token: String },
}

pub enum OAuthProvider { Google, Discord, Twitch }

pub enum License { Free, Pro, Enterprise }

pub struct User {
    pub id: String,
    pub username: String,
    pub auth_method: AuthMethod,
    pub license: License,
    pub created_at: DateTime<Utc>,
}

pub struct Session {
    pub id: String,
    pub user_id: String,
    pub license: License,
    pub expires_at: DateTime<Utc>,
}
```

## Organization

```rust
// plugins/base/organization/src/types.rs

pub struct Organization {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub member_count: usize,
}

pub struct Member {
    pub id: String,
    pub org_id: String,
    pub handle: String,
    pub rank_id: String,
    pub joined_at: DateTime<Utc>,
}

pub struct Rank {
    pub id: String,
    pub name: String,
    pub level: i32,
    pub permissions: Vec<Permission>,
}
```

## Fleet

```rust
// plugins/base/fleet/src/types.rs

pub struct Ship {
    pub id: String,
    pub model: String,
    pub manufacturer: String,
    pub insurance: Insurance,
    pub status: ShipStatus,
}

pub enum Insurance { None, Standard, LTI }
pub enum ShipStatus { Available, InUse, Maintenance }
```

---

# 💻 IMPLEMENTATION EXAMPLES

---

## Admin CLI (verseguy-admin)

**Purpose:** Manage the master server signing key and perform administrative operations from the command line.

Usage examples:

- List current key info (requires `MASTER_ADMIN_TOKEN` or `--token`):

```bash
verseguy-admin --server http://127.0.0.1:3000 --token testtoken key-list
```

- Rotate master key (server will generate a new key and return the new public key):

```bash
verseguy-admin --server http://127.0.0.1:3000 --token testtoken key-rotate
```

- Import a keypair from file:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token testtoken key-import --file ./master.key
```

- Import a keypair from base64 string:

```bash
verseguy-admin --server http://127.0.0.1:3000 --token testtoken key-import --b64 "BASE64..."
```

Environment variables used by the master server and CLI:

- `MASTER_KEY_FILE` — path to the master key (server loads or creates this file when set)
- `MASTER_ADMIN_TOKEN` — secret token used to authorize admin API calls (`x-admin-token` header)
- `MASTER_LICENSE_SECRET` — license secret used by the server (can be left default for tests)
- `MASTER_DB_PATH` — RocksDB path used by server (defaults to `./master_server_db`)

Notes:
- The CLI is implemented in `master-server/src/admin_cli.rs` and built as the binary `verseguy-admin`.
- Admin APIs:
  - `GET /admin/keys` — returns `{ exists: bool, public_key_b64: string, path: string }` (200) or 404 if not configured.
  - `POST /admin/keys/rotate` — requires `x-admin-token` header; rotates the key and returns `{ ok: true, public_key_b64: string }`.
  - `POST /admin/keys/import` — body `{"key_b64":"..."}` (base64 of 64-byte ed25519 keypair), requires `x-admin-token`.

---

## Local Authentication (Complete)

```rust
// containers/auth/src/local.rs

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use anyhow::Result;

pub struct LocalAuth {
    storage: Arc<dyn Storage>,
}

impl LocalAuth {
    pub async fn register(&self, username: String, password: String) -> Result<User> {
        // Validate
        if username.len() < 3 { anyhow::bail!("Username too short"); }
        if password.len() < 8 { anyhow::bail!("Password too short"); }
        
        // Check existing
        if self.storage.get_user_by_username(&username).await?.is_some() {
            anyhow::bail!("Username exists");
        }
        
        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)?
            .to_string();
        
        // Create user
        let user = User {
            id: Uuid::new_v4().to_string(),
            username,
            auth_method: AuthMethod::Local { username, password_hash: hash },
            license: License::Free,
            created_at: Utc::now(),
        };
        
        self.storage.save_user(&user).await?;
        Ok(user)
    }
    
    pub async fn login(&self, username: &str, password: &str) -> Result<User> {
        let user = self.storage.get_user_by_username(username).await?
            .ok_or_else(|| anyhow::anyhow!("Invalid credentials"))?;
        
        let hash = match &user.auth_method {
            AuthMethod::Local { password_hash, .. } => password_hash,
            _ => anyhow::bail!("Not a local user"),
        };
        
        let parsed = PasswordHash::new(hash)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .map_err(|_| anyhow::anyhow!("Invalid credentials"))?;
        
        Ok(user)
    }
}
```

## RocksDB Storage (Complete)

```rust
// containers/storage/src/rocksdb.rs

use rocksdb::{DB, Options};
use anyhow::Result;

pub struct RocksDBStorage {
    db: Arc<DB>,
}

impl RocksDBStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;
        Ok(Self { db: Arc::new(db) })
    }
    
    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let bytes = serde_json::to_vec(value)?;
        self.db.put(key, bytes)?;
        Ok(())
    }
    
    pub fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: for<'de> Deserialize<'de>,
    {
        match self.db.get(key)? {
            Some(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            None => Ok(None),
        }
    }
}
```

## Plugin Interface (C++)

```cpp
// core/include/IPlugin.h

#pragma once

extern "C" {
    typedef struct IPlugin {
        const char* (*get_id)(void* self);
        const char* (*get_name)(void* self);
        bool (*initialize)(void* self, void* host);
        void (*shutdown)(void* self);
        void* instance;
    } IPlugin;
    
    IPlugin* PluginInit();  // Entry point
}
```

---

# 📋 DEVELOPMENT WORKFLOW

## Daily Workflow

```yaml
Morning:
  1. ./scripts/build.sh
  2. ./scripts/test.sh
  3. Review TODO.md

During_Development:
  1. ./scripts/dev.sh (watch mode)
  2. Make changes
  3. Write tests
  4. Commit frequently

Before_Commit:
  1. All tests pass
  2. cargo fmt
  3. cargo clippy
  4. Descriptive commit message
```

## Code Quality Checklist

```yaml
Every_Function:
  ✓ Clear purpose
  ✓ Error handling
  ✓ Logging
  ✓ Tests
  ✓ Documentation (if public)

Every_Test:
  ✓ Tests one thing
  ✓ Descriptive name
  ✓ Independent
  ✓ Deterministic

Every_Commit:
  ✓ Builds successfully
  ✓ Tests pass
  ✓ Follows style
  ✓ Descriptive message
```

---

# 🎯 IMPLEMENTATION PHASES

## Week 1-2: Foundation
- Project structure
- Core DLL (C++)
- UI shell (WinUI 3)
- Basic navigation

## Week 3-4: Containers
- Auth container (local + OAuth)
- Storage container (RocksDB)
- Licensing container
- Compliance container

## Week 5-6: Base Plugins
- Plugin registry
- Organization plugin
- Fleet plugin
- Operations plugin

## Week 7-16: Advanced Features
- Pro features
- Enterprise features
- Adapters (RSI, Discord)
- Testing & polish

---

# ⚠️ CRITICAL COPILOT REMINDERS

```yaml
Before_Every_Generation:
  1. Read relevant section of this document
  2. Understand the architecture
  3. Implement REAL functionality
  4. Handle ALL errors
  5. Add logging
  6. Write tests
  7. Verify it compiles

Common_Mistakes_To_Avoid:
  ❌ Creating mocks/stubs
  ❌ Skipping error handling
  ❌ Hardcoding values
  ❌ Copy-pasting code
  ❌ Inventing architecture
  ❌ Committing broken code

Quality_Standards:
  - Production-ready from day 1
  - No technical debt
  - No "we'll fix it later"
  - Do it right the first time
```

---

# 📚 COMPLETE SPECIFICATION

**For full details, see:**
- `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md` (2400+ lines)
- Contains complete architecture
- All data types
- All implementations
- All test examples

**This document is the WORKING REFERENCE for daily development.**

---

**VERSION:** 2.0.0-local
**STATUS:** Development Active
**LAST UPDATED:** 2026-01-04

---

## 📝 Recent changes (2026-01-04) ✅
- **CTest / C++ tests**: Enabled CTest in the top-level CMake and added CTest-based CI step (run with `ctest --output-on-failure -C Release`).
- **Cross-platform scripts**: Added `scripts/build.sh` and `scripts/test.sh` (designed for WSL/Git Bash on Windows). Examples:
  - `./scripts/build.sh --release` — configure & build C++ and Rust components
  - `./scripts/test.sh` — runs `cargo test` and `ctest` where available
- **Launcher**: Added `--no-gui` / `VERSEGUY_HEADLESS` support and a launcher smoke test registered with CTest.
- **Audit log**: Added `verseguy_audit` crate (append-only SHA256 hash chain) with unit tests and verification API.
- **Compliance (GDPR)**: Implemented `export_user_data` and `delete_user_data` in `containers/compliance` with tests.
- **WASM sandbox (POC)**: Added `verseguy_wasm_sandbox` demonstrating a Wasmtime-based sandbox and tests.
- **E2E tests**: Added master-server end-to-end test covering register → login → publish → verify flow.
- **Git**: Initialized local repo and split the initial large commit into focused commits on branch `chore/init-repo-scripts` (no remote push without approval).

> **How to run tests locally** 🔧
> - Build everything: `./scripts/build.sh` (or use CMake / Cargo directly)
> - Run Rust tests: `cargo test --workspace`
> - Run C++ tests: `cmake --build build --config Release && ctest --output-on-failure -C Release`
> - Quick script: `./scripts/test.sh` (runs cargo tests then `ctest` when present)

END OF COPILOT INSTRUCTION
