---
title: VERSE_GUY_V2_ULTIMATE_SPECIFICATION
version: 2.0.0-alpha
status: SINGLE_SOURCE_OF_TRUTH
authority: ABSOLUTE
audience: GitHub Copilot + Visual Studio Code (Solo Dev)
mode: Offline-First | Enterprise-Grade | Community-Driven
author: Matthias Eckel
date: 2026-01-03
---

# ⚡ VERSE GUY V2.0 — ULTIMATE SPECIFICATION

**"The Single Source of Truth — Everything Real, Nothing Mocked"**

---

## 🎯 0. PURPOSE & AUTHORITY (READ FIRST)

### 0.1 Document Role

This document is the **ONLY authoritative specification** for Verse Guy V2.0.

**GitHub Copilot MUST:**
- Follow this document **literally**
- Generate **only** what is explicitly defined here
- **Never invent** subsystems, UI flows, or permissions
- **Never create mock data** or simulation layers
- **Always reference** relevant sections when generating code

**Any code, architecture, or design that contradicts this file is INVALID.**

### 0.2 Project Identity

```yaml
Project:
  Name: Verse Guy
  Version: 2.0.0 (Complete Rework)
  Type: Offline-First Enterprise Framework
  Domain: Star Citizen Community & Organization Management
  Philosophy: "A Cockpit, Not a Toy — A System, Not a Script"

Target_Users:
  Primary:
    - Star Citizen Players (95% Windows, Gaming PCs)
    - Organization Leaders
    - Fleet Commanders
    - Community Managers
  
  Secondary:
    - Plugin Developers
    - System Administrators
    - Enterprise IT Departments

Design_Ethos:
  Star_Citizen_Inspired:
    - Systems over Scripts
    - Simulation over Shortcuts
    - Modularity over Monoliths
    - Community Empowerment
    - Long-term Scalability
  
  Core_Values:
    - Privacy-First
    - Offline-First
    - Plugin-Driven
    - Enterprise-Grade
    - Legally Compliant
```

---

## 🏗️ 1. FUNDAMENTAL ARCHITECTURE

### 1.1 Core Design Principles (NON-NEGOTIABLE)

```yaml
Principles:
  1. Minimal_Core:
     - Core = Bootstrap + Loader + UI Shell ONLY
     - Core size: <5MB
     - Core startup: <500ms
     - NO business logic in core

  2. Everything_Is_A_Module:
     - Containers = Core infrastructure (Auth, Storage, etc.)
     - Plugins = Features (Org, Fleet, Ops, etc.)
     - Adapters = External integrations (RSI, Discord, etc.)

  3. Offline_First:
     - All features work offline
     - Sync is optional, not required
     - No online-gating of basic functionality
     - Master Server = coordination, not dependency

  4. Windows_Native_Optimized:
     - C++ Core DLL (Native Windows performance)
     - Rust DLLs (Logic + Plugins)
     - WinUI 3 + WebView2 (UI)
     - 10x faster than Electron/Tauri

  5. License_Gated_Features:
     - Free: Base functionality
     - Pro: Advanced features
     - Enterprise: RBAC, Multi-Org, Compliance
     - NO gameplay advantages
     - NO Star Citizen automation

  6. Real_Data_Only:
     - NO mocks
     - NO stubs
     - NO simulation layers
     - UI reads live runtime state
     - Everything shown is real

  7. Security_Native:
     - Capability-based permissions
     - Plugin sandboxing (WASM preferred)
     - Signed everything (plugins, themes, data)
     - Audit logs (append-only, local)
     - Kill switches (global + local)

  8. Compliance_Built_In:
     - GDPR, CCPA, DSA
     - Country-specific laws (190+ countries)
     - CIG ToS enforcement
     - Third-party API ToS compliance
     - Right to export, delete, correct
```

---

## 📊 2. WINDOWS NATIVE ARCHITECTURE

### 2.1 Why Windows Native?

```yaml
Rationale:
  Target_Audience:
    Windows: ~95%
    Linux: ~3% (Proton/Wine)
    macOS: ~2% (Star Citizen not supported)
  
  Hardware:
    Gaming_PCs: 90%+
    High-End: 60% (RTX 3070+, 32GB RAM)
    Mid-Range: 30% (GTX 1070+, 16GB RAM)
  
  Benefits:
    Performance: 10x faster startup, 6x less memory
    Integration: Native Windows 11, Fluent Design
    Updates: Granular DLL updates (KB instead of MB)
    Experience: 120 FPS UI, instant responsiveness
```

### 2.2 Hybrid DLL Architecture

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# FOUR-LAYER ARCHITECTURE
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Layer_1_Launcher:
  File: VerseguY.exe
  Size: <100KB
  Language: C++
  Purpose: Tiny stub that loads core DLL
  
  Code:
    ```cpp
    int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE, PWSTR, int) {
        HMODULE hCore = LoadLibraryW(L"VerseguY.Core.dll");
        auto initFunc = (InitFunc)GetProcAddress(hCore, "Initialize");
        initFunc(); // Loads plugins, starts UI
        return 0;
    }
    ```

Layer_2_Core:
  File: VerseguY.Core.dll
  Size: ~2MB
  Language: C++ (Native Windows)
  Purpose: Bootstrap + Plugin Loader + Windows Integration
  
  Responsibilities:
    - DLL loading/unloading
    - Plugin discovery
    - Windows API wrappers
    - Security sandbox
    - Event bus
    - UI shell initialization
  
  Forbidden:
    - Business logic
    - Domain rules
    - Feature implementation

Layer_3_Containers:
  Files: Auth.dll, Storage.dll, Licensing.dll, Compliance.dll, P2P.dll
  Size: ~50-100KB each
  Language: Rust
  Purpose: Core infrastructure services
  
  List:
    - Auth.dll (Multi-auth: Local, Google, Discord, Twitch)
    - Storage.dll (RocksDB local + Cloud sync + P2P CRDT)
    - Licensing.dll (License validation + Feature gates)
    - Compliance.dll (GDPR/CCPA/DSA + Country-specific)
    - P2P.dll (libp2p mesh + DHT + mDNS)
    - Audit.dll (Append-only logs + Integrity verification)

Layer_4_Plugins:
  Files: Plugin.*.dll
  Size: ~100-500KB each
  Language: Rust (or WASM)
  Purpose: Feature modules with sub-plugins
  
  Examples:
    - Plugin.Organization.dll
      - Sub: members (Free)
      - Sub: ranks (Free)
      - Sub: recruitment (Pro)
      - Sub: analytics (Pro)
      - Sub: rbac (Enterprise)
      - Sub: multi-org (Enterprise)
    
    - Plugin.Fleet.dll
      - Sub: hangar (Free)
      - Sub: loadouts (Free)
      - Sub: tracking (Pro)
      - Sub: analytics (Pro)
    
    - Plugin.Operations.dll
      - Sub: planning (Free)
      - Sub: missions (Free)
      - Sub: aar (Pro)
      - Sub: analytics (Pro)

Layer_5_UI:
  Technology: WinUI 3 (native shell) + WebView2 (dashboards)
  Purpose: Hybrid UI for best of both worlds
  
  WinUI3_For:
    - Application shell
    - Navigation
    - System controls
    - Native Windows 11 feel
  
  WebView2_For:
    - Complex dashboards
    - Data visualization
    - Dynamic content
    - React flexibility
```

### 2.3 Performance Benchmarks

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# NATIVE vs TAURI vs ELECTRON
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Startup_Time:
  Electron: ~3000ms
  Tauri: ~1000ms
  Native_DLL: ~300ms
  Winner: Native (10x faster)

Memory_Usage_Idle:
  Electron: ~300MB
  Tauri: ~100MB
  Native_DLL: ~50MB
  Winner: Native (6x less)

Binary_Size:
  Electron: ~150MB
  Tauri: ~15MB
  Native_DLL: ~15MB (2MB core + 10MB plugins + 3MB UI)
  Winner: Tie with Tauri

CPU_Usage_Idle:
  Electron: ~2-3%
  Tauri: ~0.5-1%
  Native_DLL: ~0.1%
  Winner: Native

File_IO_1000_Ops:
  Electron: ~100ms
  Tauri: ~50ms
  Native_DLL: ~20ms
  Winner: Native (5x faster)

UI_Frame_Time:
  Electron: ~16ms (60 FPS, drops)
  Tauri: ~16ms (60 FPS, stable)
  Native_WinUI3: ~8ms (120 FPS, smooth)
  Winner: Native

Plugin_Load_Time:
  Tauri_WASM: ~100ms per plugin
  Native_DLL: ~5ms per plugin
  Winner: Native (20x faster)

Update_Download_Single_Plugin:
  Tauri: 15MB (full binary)
  Native_DLL: 50-500KB (single DLL)
  Winner: Native (30-300x smaller)
```

---

## 🗂️ 3. PROJECT STRUCTURE

### 3.1 Complete Workspace Layout

```bash
verse-guy-v2/
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                # Local CI/CD (builds Rust & C++, runs CTest for C++ tests; uses `scripts/ci-local.ps1` locally)
│   │   ├── security-scan.yml     # Vulnerability scanning
│   │   └── compliance-check.yml  # Legal compliance validation
│   └── dependabot.yml
│
├── docs/
│   ├── architecture/
│   │   ├── ARCHITECTURE.md
│   │   ├── CONTAINERS.md
│   │   ├── PLUGINS.md
│   │   └── WINDOWS_NATIVE.md
│   ├── legal/
│   │   ├── COMPLIANCE.md
│   │   ├── GDPR.md
│   │   ├── CCPA.md
│   │   ├── DSA.md
│   │   ├── COUNTRY_LAWS.md
│   │   └── THIRD_PARTY_TOS.md
│   ├── user/
│   │   ├── USER_GUIDE.md
│   │   ├── GETTING_STARTED.md
│   │   └── FAQ.md
│   └── developer/
│       ├── PLUGIN_DEV.md
│       ├── CONTAINER_DEV.md
│       └── API_REFERENCE.md
│
├── core/                        # C++ Native Core
│   ├── VerseguY.Core/
│   │   ├── src/
│   │   │   ├── WinMain.cpp
│   │   │   ├── DllLoader.cpp
│   │   │   ├── PluginHost.cpp
│   │   │   ├── SecuritySandbox.cpp
│   │   │   └── EventBus.cpp
│   │   ├── include/
│   │   │   ├── IPlugin.h
│   │   │   ├── IPluginHost.h
│   │   │   └── Capabilities.h
│   │   └── VerseguY.Core.vcxproj
│   └── tests/
│
├── launcher/                    # Tiny Stub Executable
│   ├── main.cpp
│   └── VerseguY.vcxproj
│
├── containers/                  # Rust Infrastructure DLLs
│   ├── auth/
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── local.rs
│   │   │   ├── oauth/
│   │   │   │   ├── google.rs
│   │   │   │   ├── discord.rs
│   │   │   │   └── twitch.rs
│   │   │   └── session.rs
│   │   └── tests/
│   │
│   ├── storage/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── rocksdb.rs
│   │   │   ├── cloud_sync.rs
│   │   │   └── p2p_crdt.rs
│   │   └── tests/
│   │
│   ├── licensing/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── validator.rs
│   │   │   ├── features.rs
│   │   │   └── tiers.rs
│   │   └── tests/
│   │
│   ├── compliance/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── gdpr.rs
│   │   │   ├── ccpa.rs
│   │   │   ├── dsa.rs
│   │   │   ├── country_specific.rs
│   │   │   └── tos_validator.rs
│   │   └── tests/
│   │
│   ├── p2p/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── mesh.rs
│   │   │   ├── discovery.rs
│   │   │   ├── sync.rs
│   │   │   └── security.rs
│   │   └── tests/
│   │
│   └── audit/
│       ├── src/
│       │   ├── lib.rs
│       │   ├── logger.rs
│       │   ├── verifier.rs
│       │   └── exporter.rs
│       └── tests/
│
├── plugins/                     # Rust Feature DLLs
│   ├── registry/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── discovery.rs
│   │   │   ├── loader.rs
│   │   │   ├── dependency.rs
│   │   │   └── lifecycle.rs
│   │   └── tests/
│   │
│   ├── base/                    # Free Tier Plugins
│   │   ├── organization/
│   │   │   ├── manifest.toml
│   │   │   ├── Cargo.toml
│   │   │   ├── src/
│   │   │   │   ├── lib.rs
│   │   │   │   └── subplugins/
│   │   │   │       ├── members.rs
│   │   │   │       ├── ranks.rs
│   │   │   │       ├── recruitment.rs    # (Pro)
│   │   │   │       ├── analytics.rs      # (Pro)
│   │   │   │       ├── rbac.rs           # (Enterprise)
│   │   │   │       └── multi_org.rs      # (Enterprise)
│   │   │   └── tests/
│   │   │
│   │   ├── fleet/
│   │   │   ├── manifest.toml
│   │   │   ├── src/
│   │   │   │   └── subplugins/
│   │   │   │       ├── hangar.rs
│   │   │   │       ├── loadouts.rs
│   │   │   │       ├── tracking.rs       # (Pro)
│   │   │   │       └── analytics.rs      # (Pro)
│   │   │   └── tests/
│   │   │
│   │   └── operations/
│   │       ├── manifest.toml
│   │       ├── src/
│   │       │   └── subplugins/
│   │       │       ├── planning.rs
│   │       │       ├── missions.rs
│   │       │       ├── aar.rs            # (Pro)
│   │       │       └── analytics.rs      # (Pro)
│   │       └── tests/
│   │
│   ├── pro/                     # Pro Tier Plugins
│   │   ├── treasury/
│   │   ├── automation/
│   │   └── advanced_analytics/
│   │
│   ├── enterprise/              # Enterprise Tier Plugins
│   │   ├── rbac_extended/
│   │   ├── audit_extended/
│   │   ├── sso/
│   │   └── multi_org_manager/
│   │
│   └── adapters/                # External Integrations
│       ├── rsi/
│       │   ├── src/
│       │   │   ├── lib.rs
│       │   │   ├── oauth.rs
│       │   │   ├── session.rs
│       │   │   ├── scraper.rs
│       │   │   └── hangar.rs
│       │   └── tests/
│       │
│       ├── discord/
│       ├── fleetyards/
│       ├── erkul/
│       └── scunpacked/          # (Dev-only)
│
├── ui/                          # WinUI 3 + WebView2
│   ├── native/                  # WinUI 3 Shell
│   │   ├── VerseguY.UI/
│   │   │   ├── App.xaml
│   │   │   ├── MainWindow.xaml
│   │   │   ├── Startup/
│   │   │   │   ├── SplashScreen.xaml
│   │   │   │   └── LoadingProgress.xaml
│   │   │   ├── Onboarding/
│   │   │   │   ├── OnboardingFlow.xaml
│   │   │   │   ├── WelcomeScreen.xaml
│   │   │   │   ├── AuthSelection.xaml
│   │   │   │   └── InitialSetup.xaml
│   │   │   ├── Auth/
│   │   │   │   ├── LoginScreen.xaml
│   │   │   │   └── OAuthButtons.xaml
│   │   │   └── Shell/
│   │   │       ├── AppShell.xaml
│   │   │       ├── Navigation.xaml
│   │   │       └── Sidebar.xaml
│   │   └── VerseguY.UI.csproj
│   │
│   └── web/                     # React for WebView2
│       ├── package.json
│       ├── src/
│       │   ├── index.tsx
│       │   ├── tabs/
│       │   │   ├── DashboardTab.tsx
│       │   │   ├── OrganizationTab.tsx
│       │   │   ├── FleetTab.tsx
│       │   │   ├── OperationsTab.tsx
│       │   │   ├── TreasuryTab.tsx
│       │   │   └── PluginsTab.tsx
│       │   ├── components/
│       │   └── hooks/
│       └── public/
│
├── master-server/               # Admin/Dev Control Plane
│   ├── src/
│   │   ├── main.rs
│   │   ├── modules/
│   │   │   ├── auth.rs
│   │   │   ├── licensing.rs
│   │   │   ├── verification.rs
│   │   │   ├── p2p_bootstrap.rs
│   │   │   ├── update.rs
│   │   │   ├── plugin_registry.rs
│   │   │   ├── bug_tracker.rs
│   │   │   ├── audit_vault.rs
│   │   │   ├── statistics.rs
│   │   │   └── announcements.rs
│   │   └── api/
│   └── tests/
│
├── installer/                   # OS-Specific Installers
│   ├── windows/
│   │   ├── verseguy.wxs         # WiX MSI
│   │   └── build.ps1
│   ├── macos/
│   │   ├── Info.plist
│   │   └── build.sh
│   └── linux/
│       ├── debian/
│       └── rpm/
│
├── scripts/
│   ├── build.sh
│   ├── test.sh
│   ├── release.sh
│   ├── unify-versions.sh
│   └── compliance-check.sh
│
├── legal/
│   ├── ToS.md
│   ├── PrivacyPolicy.md
│   ├── GDPR_DPA.md
│   ├── ThirdPartyNotices.md
│   └── CountrySpecific/
│       ├── EU.md
│       ├── US.md
│       ├── UK.md
│       └── ...
│
├── Cargo.toml                   # Workspace root
├── README.md
├── LICENSE
├── CHANGELOG.md
└── VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md  # THIS FILE
```

---

## 🚀 4. STARTUP SEQUENCE

### 4.1 Complete Startup Flow

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# STREAMLINED, INSTANT, TRANSPARENT
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Time_0ms:
  User_Launches: VerseguY.exe
  Action: Tiny stub (<100KB) executes

Time_50ms:
  Core_Loads: LoadLibrary("VerseguY.Core.dll")
  Action: C++ native core initializes

Time_100ms:
  UI_Shell_Loads: WinUI 3 window appears
  Display: Animated logo splash screen (Mobiglas-style)
  Action: Initialize WebView2 runtime

Time_200ms:
  Startup_Routine:
    - Check config directory
    - Discover available plugins
    - Load plugin registry
    - Check first-run marker
  
  Progress_Display:
    - Animated progress bar
    - Status text ("Initializing...", "Loading plugins...")

Time_300ms:
  Decision_Point:
    IF first_run:
      Navigate: /onboarding
    ELSE:
      Navigate: /login

Time_500ms:
  Ready_State: UI fully loaded
  
  Onboarding_OR_Login:
    - User completes setup
    - System loads licensed plugins
    - Navigate to /dashboard

Total_Time_To_UI: ~500ms
Total_Time_To_Dashboard: User-dependent (onboarding/login)
```

### 4.2 First-Run Detection

```rust
// File: core/src/startup.rs

use std::path::PathBuf;
use std::fs;

pub fn is_first_run() -> Result<bool> {
    let config_dir = get_config_dir()?;
    let marker_file = config_dir.join(".initialized");
    Ok(!marker_file.exists())
}

pub fn mark_initialized() -> Result<()> {
    let config_dir = get_config_dir()?;
    fs::create_dir_all(&config_dir)?;
    fs::write(config_dir.join(".initialized"), "")?;
    Ok(())
}

fn get_config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or(Error::NoConfigDir)?
        .join("verseguy");
    Ok(config_dir)
}
```

---

## 🔐 5. AUTHENTICATION ARCHITECTURE

### 5.1 Multi-Auth System

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# FOUR AUTH METHODS
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Method_1_Local_Auth:
  Flow: Username + Password → Local storage
  
  Storage:
    - Password: Argon2 hashed
    - Session: JWT (24h expiry)
    - Data: Local only, no cloud
  
  Features:
    ✓ Organization management (basic)
    ✓ Fleet management (manual entry)
    ✓ Operations planning
    ❌ No RSI sync
    ❌ No cloud backup
    ❌ No external integrations
  
  License: Always Free
  
  Use_Case:
    - Privacy-focused users
    - No internet connection
    - Testing/development
    - Offline-only operations

Method_2_OAuth_Google:
  Flow: Google OAuth 2.0 → Access token
  
  Imports:
    - Gmail contacts (optional)
    - Google Calendar events (optional)
    - Google Drive files (optional)
  
  Features:
    ✓ All local features
    ✓ RSI hangar sync
    ✓ Cloud backup
    ✓ Cross-device sync
    ✓ Calendar integration
    ✓ Drive integration
  
  License: Free/Pro/Enterprise (user's choice)
  
  Use_Case:
    - Google Workspace users
    - Want calendar integration
    - Cloud backup desired
    - Multi-device usage

Method_3_OAuth_Discord:
  Flow: Discord OAuth 2.0 → Access token
  
  Imports:
    - Discord profile
    - Server memberships
    - Roles (for auto-rank mapping)
  
  Features:
    ✓ All OAuth features
    ✓ Discord server auto-link
    ✓ Webhook integration
    ✓ Role synchronization
    ✓ Bot commands (optional)
  
  License: Free/Pro/Enterprise
  
  Use_Case:
    - Gaming communities
    - Discord-first organizations
    - Want webhook integration
    - Role-based automation

Method_4_OAuth_Twitch:
  Flow: Twitch OAuth 2.0 → Access token
  
  Imports:
    - Twitch profile
    - Stream schedule
    - Follower count
  
  Features:
    ✓ All OAuth features
    ✓ Stream scheduling
    ✓ Go-live notifications
    ✓ Follower integration
    ✓ VOD archiving
  
  License: Free/Pro/Enterprise
  
  Use_Case:
    - Content creators
    - Streaming organizations
    - Want Twitch integration
    - Stream-focused operations
```

### 5.2 Feature Matrix by Auth Method

```yaml
Feature_Comparison:

  Feature                     | Local | OAuth
  ----------------------------|-------|-------
  Organization Management     | ✓     | ✓
  Fleet Management (manual)   | ✓     | ✓
  Operations Planning         | ✓     | ✓
  RSI Hangar Sync             | ✗     | ✓
  Cloud Backup                | ✗     | ✓
  Cross-Device Sync           | ✗     | ✓
  External Integrations       | ✗     | ✓
  Advanced Plugins            | ✗     | ✓ (if licensed)
  Calendar Integration        | ✗     | ✓ (Google only)
  Discord Webhooks            | ✗     | ✓ (Discord only)
  Stream Scheduling           | ✗     | ✓ (Twitch only)
```

### 5.3 Session Management

```rust
// File: containers/auth/src/session.rs

use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub user_id: String,
    pub auth_method: AuthMethod,
    pub license: License,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthMethod {
    Local,
    OAuthGoogle { token: String },
    OAuthDiscord { token: String },
    OAuthTwitch { token: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd)]
pub enum License {
    Free,
    Pro,
    Enterprise,
}

pub struct AuthContainer {
    jwt_secret: Vec<u8>,
    storage: Arc<StorageContainer>,
}

impl AuthContainer {
    pub async fn create_session(
        &self,
        user_id: String,
        auth_method: AuthMethod,
        license: License,
    ) -> Result<String> {
        let session = Session {
            user_id,
            auth_method,
            license,
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::days(30),
        };
        
        let token = encode(
            &Header::default(),
            &session,
            &EncodingKey::from_secret(&self.jwt_secret),
        )?;
        
        // Store session in local database
        self.storage.save_session(&session).await?;
        
        Ok(token)
    }
    
    pub async fn validate_session(&self, token: &str) -> Result<Session> {
        let decoded = decode::<Session>(
            token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &Validation::default(),
        )?;
        
        // Check expiry
        if decoded.claims.expires_at < Utc::now() {
            return Err(Error::SessionExpired);
        }
        
        Ok(decoded.claims)
    }
}
```

---

## 📦 6. PLUGIN ARCHITECTURE

### 6.1 Plugin-in-Plugin System

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# GRANULAR CONTROL
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Concept: Every plugin contains sub-plugins for granular control

Example_Organization_Plugin:

  Plugin: "Organization Management"
  Version: 2.0.0
  License_Required: Free
  
  Sub-Plugins:
    
    members:
      Name: "Member Management"
      License: Free
      Enabled_By_Default: true
      Capabilities:
        - Add/edit/delete members
        - View member list
        - Assign ranks
    
    ranks:
      Name: "Rank System"
      License: Free
      Enabled_By_Default: true
      Capabilities:
        - Create/edit/delete ranks
        - Define rank hierarchy
        - Set rank permissions
    
    recruitment:
      Name: "Recruitment System"
      License: Pro
      Enabled_By_Default: false
      Capabilities:
        - Application system
        - Interview scheduling
        - Auto-accept rules
        - Candidate tracking
    
    analytics:
      Name: "Organization Analytics"
      License: Pro
      Enabled_By_Default: false
      Capabilities:
        - Member activity tracking
        - Retention metrics
        - Growth charts
        - Engagement scores
    
    rbac:
      Name: "Role-Based Access Control"
      License: Enterprise
      Enabled_By_Default: false
      Capabilities:
        - Fine-grained permissions
        - Role templates
        - Permission inheritance
        - Audit logging
    
    multi_org:
      Name: "Multi-Organization Support"
      License: Enterprise
      Enabled_By_Default: false
      Capabilities:
        - Manage multiple organizations
        - Cross-org operations
        - Unified dashboard
        - Inter-org communication
```

### 6.2 Plugin Manifest Format

```toml
# File: plugins/base/organization/manifest.toml

[plugin]
id = "org.verseguy.organization"
name = "Organization Management"
version = "2.0.0"
author = "Verse Guy Team"
description = "Complete organization management system"
license_required = "Free"
core_version_min = "2.0.0"
sdk_version = "1.0.0"

[capabilities]
required = [
    "storage:read",
    "storage:write",
    "ui:panel",
    "network:p2p",
]

optional = [
    "network:master_server",
    "notifications:system",
]

[subplugins.members]
name = "Member Management"
enabled_by_default = true
license_required = "Free"

[subplugins.ranks]
name = "Rank System"
enabled_by_default = true
license_required = "Free"

[subplugins.recruitment]
name = "Recruitment System"
enabled_by_default = false
license_required = "Pro"

[subplugins.analytics]
name = "Organization Analytics"
enabled_by_default = false
license_required = "Pro"

[subplugins.rbac]
name = "Role-Based Access Control"
enabled_by_default = false
license_required = "Enterprise"

[subplugins.multi_org]
name = "Multi-Organization Support"
enabled_by_default = false
license_required = "Enterprise"
```

### 6.3 Plugin SDK Interface

```rust
// File: core/include/IPlugin.h (C++ header)

#pragma once
#include <stdint.h>

extern "C" {
    // Plugin capabilities
    enum Capability {
        CAP_STORAGE_READ = 1 << 0,
        CAP_STORAGE_WRITE = 1 << 1,
        CAP_NETWORK_P2P = 1 << 2,
        CAP_NETWORK_MASTER = 1 << 3,
        CAP_UI_PANEL = 1 << 4,
        CAP_NOTIFICATIONS = 1 << 5,
    };
    
    // Plugin interface
    struct IPlugin {
        const char* (*get_id)(void*);
        const char* (*get_name)(void*);
        const char* (*get_version)(void*);
        uint64_t (*get_required_capabilities)(void*);
        bool (*initialize)(void*, void* host);
        void (*shutdown)(void*);
    };
    
    // Plugin host interface
    struct IPluginHost {
        void* (*get_storage_service)(void*);
        void* (*get_network_service)(void*);
        void* (*get_ui_service)(void*);
        bool (*has_capability)(void*, uint64_t cap);
        void (*log)(void*, const char* level, const char* message);
    };
    
    // Plugin entry point
    IPlugin* PluginInit();
}
```

```rust
// File: plugins/base/organization/src/lib.rs (Rust implementation)

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

pub struct OrganizationPlugin {
    id: CString,
    name: CString,
    version: CString,
    host: *mut c_void,
}

impl OrganizationPlugin {
    pub fn new() -> Self {
        Self {
            id: CString::new("org.verseguy.organization").unwrap(),
            name: CString::new("Organization Management").unwrap(),
            version: CString::new("2.0.0").unwrap(),
            host: std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn PluginInit() -> *mut IPlugin {
    let plugin = Box::new(OrganizationPlugin::new());
    let interface = Box::new(create_plugin_interface(plugin));
    Box::into_raw(interface)
}

// ... implementation details
```

---

## 🌐 7. MASTER SERVER ARCHITECTURE

### 7.1 Purpose & Philosophy

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# MASTER SERVER = COORDINATION, NOT DEPENDENCY
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Philosophy: "The Control Tower, Not the Flight Computer"

Mental_Model:
  Client: Spaceship
  P2P: Radio between ships
  Master_Server: Space station control tower
  
  Control_Tower:
    ✓ Issues identities (IDs, licenses)
    ✓ Verifies ships (plugins, signatures)
    ✓ Blocks dangerous ships (revocations)
    ✓ Coordinates traffic (P2P bootstrap)
    ❌ Does NOT control flight
    ❌ Does NOT store private cargo
    ❌ Does NOT gate takeoff

If_Master_Server_Is_Down:
  Clients: Continue working
  P2P: Continues functioning
  Features: No blocking
  Only_Paused:
    - New identity issuing
    - New plugin verification
    - Coordinated discovery
    - Update distribution

Access:
  Who_Can_Access:
    - System Administrators
    - Developers
    - Automated services (CI/CD)
  
  Who_CANNOT_Access:
    - Regular users
    - Organization leaders
    - Plugin developers (read-only registry access)
```

### 7.2 Master Server Modules

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# COMPLETE MODULE BREAKDOWN
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MasterServer:
  
  Module_1_Auth_Identity_Server:
    Purpose: Manage user identities and authentication
    
    Responsibilities:
      - Issue OAuth tokens
      - Validate authentication
      - Manage user accounts
      - Session lifecycle
    
    Storage:
      - User profiles
      - OAuth provider links
      - Login history
    
    API:
      POST /auth/register
      POST /auth/login
      POST /auth/refresh
      GET  /auth/validate
      POST /auth/logout
  
  Module_2_License_Entitlement_Server:
    Purpose: Manage licenses and feature entitlements
    
    Responsibilities:
      - Issue signed license tokens
      - Validate license validity
      - Track license assignments
      - Handle upgrades/downgrades
    
    License_Types:
      Free:
        - Core runtime
        - Community plugins
        - Offline usage
      
      Pro:
        - Extended plugins
        - Advanced features
        - Priority support
      
      Enterprise:
        - RBAC
        - Multi-org
        - Compliance dashboards
        - SLA guarantees
    
    Offline_Support:
      - Licenses cached locally
      - Grace period: 30 days
      - Manual renewal if offline
      - No feature blocking
    
    API:
      GET  /license/validate
      POST /license/activate
      GET  /license/features
      POST /license/upgrade
  
  Module_3_Verification_Revocation_Registry:
    Purpose: Plugin and client trust management (KILL SWITCH)
    
    Responsibilities:
      - Sign plugin releases
      - Verify plugin signatures
      - Revoke compromised plugins
      - Block malicious versions
      - Blacklist problematic clients
    
    Verification_Levels:
      VerseGuy_Signed:
        - Official Verse Guy plugins
        - Highest trust level
        - Automatic approval
      
      Owner_Signed:
        - Developer-verified plugins
        - Medium trust level
        - Manual review process
      
      Org_Signed:
        - Organization-specific plugins
        - Low trust level
        - Organization-only distribution
    
    Revocation_Triggers:
      - Security vulnerability
      - ToS violation
      - CIG complaint
      - Malicious behavior
      - Legal requirement
    
    API:
      POST /verify/plugin
      GET  /verify/status/:plugin_id
      POST /verify/revoke
      GET  /verify/revocations
  
  Module_4_P2P_Bootstrap_Directory:
    Purpose: Assist P2P peer discovery
    
    Responsibilities:
      - Provide initial peer list
      - NAT traversal assistance
      - Relay coordination
      - Region-aware routing
    
    Discovery_Methods:
      LAN: mDNS (automatic)
      WAN: DHT (decentralized)
      Bootstrap: Master server (optional)
    
    If_Unavailable:
      - LAN discovery works
      - DHT discovery works
      - Only cross-region affected
    
    API:
      GET  /p2p/bootstrap/peers
      POST /p2p/bootstrap/announce
      GET  /p2p/bootstrap/relay
  
  Module_5_Update_Patch_Authority:
    Purpose: Distribute updates safely
    
    Responsibilities:
      - Host update artifacts
      - Sign updates
      - Provide delta patches
      - Rollback capability
    
    Update_Types:
      Core: Full binary updates
      Containers: Individual DLL updates
      Plugins: Individual plugin updates
      UI: WebView2 bundle updates
    
    Delta_Updates:
      - Binary diff (bsdiff)
      - Only changed DLLs
      - Bandwidth efficient
      - Integrity verified
    
    API:
      GET  /update/check
      GET  /update/download/:version
      GET  /update/delta/:from/:to
      POST /update/complete
  
  Module_6_Plugin_Registry:
    Purpose: Central plugin discovery and metadata
    
    Responsibilities:
      - Host plugin metadata
      - Track versions
      - Provide search
      - Analytics (downloads, ratings)
    
    Metadata:
      - Plugin manifest
      - Screenshots
      - Documentation
      - Compatibility matrix
      - User ratings
    
    API:
      GET  /plugins/search
      GET  /plugins/:id
      GET  /plugins/:id/versions
      POST /plugins/publish (devs only)
  
  Module_7_Bug_Crash_Intake:
    Purpose: Collect crash reports and diagnostics
    
    Responsibilities:
      - Receive crash dumps
      - Parse stack traces
      - Deduplicate reports
      - Prioritize issues
      - Privacy filtering
    
    Privacy:
      - Strip personal data
      - Hash identifiers
      - Aggregate only
      - User opt-in required
    
    API:
      POST /bugs/crash
      POST /bugs/report
      GET  /bugs/stats (admins only)
  
  Module_8_Audit_Vault:
    Purpose: Store compliance audit logs
    
    Responsibilities:
      - Archive client audit logs
      - Verify integrity
      - GDPR compliance
      - Legal discovery support
    
    Storage:
      - Append-only
      - Hash-chained
      - Encrypted at rest
      - Time-stamped
    
    Retention:
      EU: 7 years
      US: 5 years
      Other: Per-country rules
    
    API:
      POST /audit/submit
      GET  /audit/verify/:hash
      GET  /audit/export/:user_id (GDPR)
  
  Module_9_Statistics_Telemetry:
    Purpose: Collect anonymous usage metrics (OPT-IN)
    
    Opt_In_Required: true
    
    Collected:
      - Client version
      - OS version
      - Plugin usage (counts only)
      - Performance metrics
      - Feature usage
    
    NOT_Collected:
      - Personal data
      - Organization data
      - Chat logs
      - User content
    
    API:
      POST /stats/metrics
      GET  /stats/dashboard (admins only)
  
  Module_10_Announcement_Marketing_Feed:
    Purpose: Distribute news and announcements
    
    Responsibilities:
      - Release notes
      - Feature announcements
      - Maintenance notices
      - Security advisories
    
    API:
      GET  /news/latest
      GET  /news/:id
```

### 7.3 Master Server API Authentication

```yaml
API_Authentication:

  Client_API:
    Method: JWT Bearer Token
    Issuer: Auth & Identity Server
    Expiry: 24 hours
    Refresh: Automatic (before expiry)
  
  Admin_API:
    Method: mTLS (Mutual TLS)
    Certificate: Admin certificate required
    2FA: Required for sensitive operations
  
  Service_API:
    Method: Service account tokens
    Scope: Limited per service
    Rotation: Weekly
```

---

## 🔐 8. SECURITY & COMPLIANCE

### 8.1 Security Architecture

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# SECURITY-FIRST DESIGN
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Security_Layers:

  Layer_1_Capability_System:
    Principle: Zero trust, explicit grants only
    
    Default: NO permissions
    
    Capabilities:
      - storage:read
      - storage:write
      - network:p2p
      - network:master_server
      - ui:panel
      - ui:notification
      - system:filesystem_read
      - system:filesystem_write
    
    Grant_Process:
      1. Plugin declares in manifest
      2. User reviews during install
      3. User approves/denies
      4. Core enforces at runtime
    
    Revocation:
      - User can revoke anytime
      - Automatic on plugin uninstall
      - Forced on security incident
  
  Layer_2_Plugin_Sandboxing:
    Technology: WASM (preferred) or Native DLL (trusted)
    
    WASM_Plugins:
      ✓ Memory isolated
      ✓ No direct system access
      ✓ Capability API only
      ✓ Safe by design
    
    Native_DLL_Plugins:
      ⚠️ Full system access
      ⚠️ Requires signing
      ⚠️ Trust-on-first-use
      ⚠️ User warning on install
  
  Layer_3_Code_Signing:
    All_Signed:
      - Core DLL (VerseGuy certificate)
      - Container DLLs (VerseGuy certificate)
      - Official plugins (VerseGuy certificate)
      - Third-party plugins (Developer certificate)
      - Themes (Author certificate)
      - Languages (Author certificate)
    
    Verification:
      - Signature checked on load
      - Certificate validated against Master Server
      - Revocation list checked
      - Load blocked if invalid
  
  Layer_4_Data_Signing:
    Signed_Data:
      - Audit log entries
      - P2P CRDT operations
      - Configuration changes
      - License tokens
    
    Hash_Chain:
      - Each entry contains hash of previous
      - Tamper detection
      - Integrity verification
      - Forensic audit trail
  
  Layer_5_Network_Security:
    P2P:
      - End-to-end encryption (TLS 1.3)
      - Identity-bound peers
      - Trust scoring
      - Rate limiting
    
    Master_Server:
      - HTTPS only
      - Certificate pinning
      - API rate limiting
      - DDoS protection
  
  Layer_6_Local_Storage_Security:
    Encryption:
      - At-rest encryption (AES-256)
      - Key derivation (PBKDF2)
      - User password = master key
    
    Backup:
      - Encrypted backups
      - Secure deletion
      - Tamper detection
```

### 8.2 Compliance Framework

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# LEGAL COMPLIANCE BUILT-IN
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Compliance_Engine:

  GDPR (EU):
    Applies_To: EU residents
    
    Requirements:
      ✓ Right to access (export all data)
      ✓ Right to deletion (full wipe)
      ✓ Right to rectification (edit data)
      ✓ Right to portability (JSON/CSV export)
      ✓ Right to object (opt-out tracking)
      ✓ Consent management (granular)
      ✓ Data minimization
      ✓ Purpose limitation
      ✓ Storage limitation (auto-delete old data)
    
    Implementation:
      - Region detection (IP geolocation)
      - Consent banners
      - Data export API
      - Data deletion API
      - Audit logging
  
  CCPA (California):
    Applies_To: California residents
    
    Requirements:
      ✓ Right to know (what data collected)
      ✓ Right to delete
      ✓ Right to opt-out (sale of data)
      ✓ Right to non-discrimination
      ✓ Disclosure requirements
    
    Implementation:
      - "Do Not Sell My Data" button
      - Privacy policy disclosure
      - Data deletion within 45 days
  
  DSA (EU Digital Services Act):
    Applies_To: EU operations
    
    Requirements:
      ✓ Content moderation
      ✓ Reporting system
      ✓ Transparency reports
      ✓ Illegal content removal
      ✓ User appeals process
    
    Implementation:
      - Report abuse button
      - Moderation queue
      - Transparency dashboard
      - Appeal workflow
  
  Country_Specific_Laws:
    Supported_Countries: 190+
    
    Examples:
      UK_GDPR: Similar to EU GDPR
      Australia_Privacy_Act: Data breach notification
      Japan_APPI: Cross-border data transfer rules
      Brazil_LGPD: Similar to GDPR
      Canada_PIPEDA: Consent requirements
    
    Implementation:
      - Per-country handlers
      - Automatic rule application
      - Legal team review
  
  CIG_ToS (Star Citizen):
    Requirements:
      ❌ No automated trading
      ❌ No gameplay automation
      ❌ No botting
      ❌ No exploits
      ❌ No real-money trading facilitation
      ✓ Fan project rules compliance
      ✓ No official endorsement implied
    
    Implementation:
      - ToS acceptance required
      - Disclaimer in UI
      - No ingame automation features
      - Community/meta tools only
  
  Third_Party_ToS:
    RSI_API:
      - Rate limiting (100 req/hour)
      - No scraping beyond OAuth
      - Attribution required
    
    Discord_API:
      - Rate limiting respected
      - No spam
      - Bot approval if needed
    
    Google_APIs:
      - Usage limits
      - Privacy policy required
      - Data use restrictions
    
    Twitch_API:
      - Rate limiting
      - No automated follows
      - Developer agreement
    
    FleetYards_API:
      - Attribution required
      - No commercial use
      - Rate limiting
    
    Erkul_Games:
      - Credit required
      - No data mirroring
```

### 8.3 Audit Logging

```rust
// File: containers/audit/src/lib.rs

use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub actor_id: String,
    pub resource_type: String,
    pub resource_id: String,
    pub action: String,
    pub changes: serde_json::Value,
    pub hash: String,
    pub previous_hash: String,
}

pub struct AuditService {
    db: Arc<RocksDB>,
    previous_hash: Arc<Mutex<String>>,
}

impl AuditService {
    pub fn log_event(&self, event: AuditEvent) -> Result<()> {
        let mut prev_hash = self.previous_hash.lock().unwrap();
        
        // Create entry
        let entry = AuditEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type: event.event_type,
            actor_id: event.actor_id,
            resource_type: event.resource_type,
            resource_id: event.resource_id,
            action: event.action,
            changes: event.changes,
            hash: String::new(), // Calculated next
            previous_hash: prev_hash.clone(),
        };
        
        // Calculate hash (includes previous hash for chain)
        let entry_json = serde_json::to_string(&entry)?;
        let mut hasher = Sha256::new();
        hasher.update(entry_json.as_bytes());
        hasher.update(prev_hash.as_bytes());
        let current_hash = format!("{:x}", hasher.finalize());
        
        // Update entry with hash
        let mut entry = entry;
        entry.hash = current_hash.clone();
        
        // Save to database
        let key = format!("audit:{}", entry.id);
        self.db.put(key.as_bytes(), serde_json::to_vec(&entry)?)?;
        
        // Update previous hash for next entry
        *prev_hash = current_hash;
        
        Ok(())
    }
    
    pub fn verify_integrity(&self) -> Result<bool> {
        // Verify entire hash chain
        let entries = self.get_all_entries()?;
        let mut prev_hash = String::new();
        
        for entry in entries {
            let entry_json = serde_json::to_string(&AuditEntry {
                hash: String::new(),
                ..entry.clone()
            })?;
            
            let mut hasher = Sha256::new();
            hasher.update(entry_json.as_bytes());
            hasher.update(prev_hash.as_bytes());
            let calculated = format!("{:x}", hasher.finalize());
            
            if calculated != entry.hash {
                return Ok(false); // Tampered!
            }
            
            prev_hash = entry.hash;
        }
        
        Ok(true)
    }
    
    pub fn export_for_user(&self, user_id: &str) -> Result<Vec<AuditEntry>> {
        // GDPR Article 20: Right to data portability
        let entries = self.db.prefix_scan(b"audit:")?
            .filter_map(|(_, value)| {
                serde_json::from_slice::<AuditEntry>(&value).ok()
            })
            .filter(|entry| entry.actor_id == user_id)
            .collect();
        
        Ok(entries)
    }
}
```

---

## 🎨 9. UI ARCHITECTURE

### 9.1 UI Philosophy

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# UI = ADAPTER LAYER, NOT BUSINESS LOGIC
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Core_Principle: "UI displays state, it does not decide state"

UI_Role:
  ✓ Observe runtime state
  ✓ Display data
  ✓ Accept user input
  ✓ Send commands to containers/plugins
  ❌ Calculate business logic
  ❌ Store authoritative data
  ❌ Bypass permission checks

Data_Flow:
  Container/Plugin → Event Bus → UI (display)
  UI → Event Bus → Container/Plugin (command)

Visual_Identity:
  Inspiration: Star Citizen Mobiglas
  Theme: Dark-first, space-themed
  Colors:
    Primary: Deep blue (#0A1628)
    Accent: Cyan (#00D9FF)
    Warning: Orange (#FF6B00)
    Error: Red (#FF0000)
    Success: Green (#00FF88)
  
  Typography:
    Headings: Orbitron (futuristic)
    Body: Inter (readable)
    Monospace: JetBrains Mono (data)
```

### 9.2 Consolidated Tab Structure

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# SMART TABS: License-aware visibility
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Main_Tabs:

  1. Dashboard:
     Visibility: Always
     License: Free
     
     Content:
       - System status
       - Quick actions
       - Recent activity
       - Notifications
       - Performance metrics
  
  2. Organization:
     Visibility: Always
     License: Free (base), Pro (extended), Enterprise (full)
     
     Sections:
       Members (Free):
         - Add/edit/delete members
         - Assign ranks
         - View member list
       
       Ranks (Free):
         - Create rank hierarchy
         - Define permissions
         - Assign roles
       
       Recruitment (Pro):
         🔒 Upgrade to Pro
         - Application system
         - Interview scheduling
         - Auto-accept rules
       
       Analytics (Pro):
         🔒 Upgrade to Pro
         - Activity tracking
         - Retention metrics
         - Growth charts
       
       RBAC (Enterprise):
         🔒 Upgrade to Enterprise
         - Fine-grained permissions
         - Role templates
         - Permission inheritance
       
       Multi-Org (Enterprise):
         🔒 Upgrade to Enterprise
         - Manage multiple orgs
         - Cross-org operations
         - Unified dashboard
  
  3. Fleet:
     Visibility: Always
     License: Free (base), Pro (extended)
     
     Sections:
       Hangar (Free):
         - Ship list
         - Manual entry
         - RSI sync (if OAuth)
       
       Loadouts (Free):
         - Ship configurations
         - Weapon loadouts
         - Component management
       
       Tracking (Pro):
         🔒 Upgrade to Pro
         - Location tracking
         - Availability status
         - Maintenance schedules
       
       Analytics (Pro):
         🔒 Upgrade to Pro
         - Fleet composition
         - Usage statistics
         - Value tracking
  
  4. Operations:
     Visibility: Always
     License: Free (base), Pro (extended)
     
     Sections:
       Planning (Free):
         - Create operations
         - Schedule events
         - Assign roles
       
       Missions (Free):
         - Mission tracking
         - Progress updates
         - Completion status
       
       AAR (Pro):
         🔒 Upgrade to Pro
         - After-action reports
         - Performance analysis
         - Lessons learned
       
       Analytics (Pro):
         🔒 Upgrade to Pro
         - Operation success rate
         - Member participation
         - Efficiency metrics
  
  5. Treasury:
     Visibility: Pro+
     License: Pro (base), Enterprise (extended)
     
     Sections:
       Transactions (Pro):
         - Income/expenses
         - Member contributions
         - Transaction history
       
       Reports (Pro):
         - Financial statements
         - Budget vs actual
         - Trend analysis
       
       Budgets (Enterprise):
         🔒 Upgrade to Enterprise
         - Budget planning
         - Allocation tracking
         - Forecasting
  
  6. Plugins:
     Visibility: Always
     License: Free
     
     Sections:
       Available:
         - Plugin marketplace
         - Search/filter
         - Install/update
       
       Installed:
         - Manage plugins
         - Enable/disable sub-plugins
         - View permissions
       
       Settings:
         - Per-plugin configuration
         - Capability management
  
  7. Settings:
     Visibility: Always
     License: Free
     
     Sections:
       User:
         - Profile settings
         - Avatar
         - Preferences
       
       Application:
         - Theme
         - Language
         - Notifications
         - Auto-update
       
       License:
         - Current license
         - Features available
         - Upgrade options
       
       Privacy:
         - Data export
         - Data deletion
         - Consent management
       
       About:
         - Version info
         - Credits
         - Legal

Dynamic_Content_Rules:
  - Sections locked by license show upgrade prompt
  - Upgrade prompt is non-intrusive (single line with button)
  - No fake content or grayed-out features
  - Clear explanation of what's locked and why
```

### 9.3 UI Implementation Example

```tsx
// File: ui/web/src/tabs/OrganizationTab.tsx

import React from 'react';
import { useAuth } from '../hooks/useAuth';
import { usePlugin } from '../hooks/usePlugin';

export function OrganizationTab() {
    const { license } = useAuth();
    const { subplugins } = usePlugin('organization');
    
    return (
        <div className="tab-container">
            <h1 className="text-3xl font-bold mb-6">Organization</h1>
            
            {/* Always visible (Free) */}
            <Section title="Members">
                <MemberList />
                <AddMemberButton />
            </Section>
            
            <Section title="Ranks">
                <RankManagement />
            </Section>
            
            {/* Pro tier */}
            {license >= 'Pro' ? (
                <>
                    <Section title="Recruitment">
                        <RecruitmentSystem />
                    </Section>
                    
                    <Section title="Analytics">
                        <OrganizationAnalytics />
                    </Section>
                </>
            ) : (
                <UpgradeSection
                    features={['Recruitment System', 'Organization Analytics']}
                    requiredLicense="Pro"
                />
            )}
            
            {/* Enterprise tier */}
            {license === 'Enterprise' ? (
                <>
                    <Section title="Access Control">
                        <RBACManagement />
                    </Section>
                    
                    <Section title="Multi-Organization">
                        <MultiOrgDashboard />
                    </Section>
                </>
            ) : license === 'Pro' ? (
                <UpgradeSection
                    features={['Role-Based Access Control', 'Multi-Organization Support']}
                    requiredLicense="Enterprise"
                />
            ) : null}
        </div>
    );
}

function UpgradeSection({ features, requiredLicense }) {
    return (
        <div className="upgrade-prompt">
            <div className="flex items-center justify-between p-4 bg-blue-900/20 border border-cyan-500/30 rounded-lg">
                <div>
                    <p className="text-sm text-gray-300 mb-1">
                        Unlock more features:
                    </p>
                    <ul className="text-xs text-gray-400">
                        {features.map(f => (
                            <li key={f}>• {f}</li>
                        ))}
                    </ul>
                </div>
                <button className="px-4 py-2 bg-cyan-600 hover:bg-cyan-700 rounded text-white text-sm">
                    Upgrade to {requiredLicense}
                </button>
            </div>
        </div>
    );
}
```

---

## 🔧 10. COPILOT BEHAVIOR RULES

### 10.1 Mandatory Rules

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# COPILOT MUST FOLLOW THESE RULES STRICTLY
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Copilot_MUST:
  1. Follow this document literally
  2. Generate only what is explicitly defined here
  3. Never invent subsystems, UI, or permissions
  4. Never create mock data or simulation layers
  5. Always reference relevant sections when generating code
  6. Ask before changing architecture
  7. Prefer composition over inheritance
  8. Prefer plugins over core changes
  9. Assume enterprise-grade requirements
  10. Write auditable, testable code

Copilot_MUST_NOT:
  1. Collapse modules
  2. Skip compliance checks
  3. Assume cloud dependency
  4. Hardcode secrets
  5. Invent APIs
  6. Create mocks or stubs
  7. Bypass security checks
  8. Skip documentation
  9. Ignore versioning
  10. Make breaking changes without approval

Code_Generation_Rules:
  - Always include error handling
  - Always include logging
  - Always include tests
  - Always document public APIs
  - Always validate inputs
  - Always check permissions
  - Prefer type safety
  - Prefer immutability
  - Prefer functional patterns where appropriate

Security_Rules:
  - Never log sensitive data
  - Never store passwords in plaintext
  - Always use prepared statements (SQL injection prevention)
  - Always validate user input
  - Always sanitize output
  - Always use HTTPS
  - Always verify signatures
  - Always check capabilities

Performance_Rules:
  - Prefer lazy loading
  - Cache where appropriate
  - Minimize allocations
  - Use async where possible
  - Profile before optimizing
  - Measure, don't guess
```

---

## 📅 11. IMPLEMENTATION TIMELINE

### 11.1 16-Week Plan

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# GREENFIELD DEVELOPMENT TIMELINE
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Week_1_2_Foundation:
  - Project structure setup
  - C++ core DLL (minimal bootstrap)
  - DLL loader system
  - Plugin interface definitions
  - WinUI 3 shell skeleton
  - Documentation foundation

Week_3_4_Containers:
  - Auth container (all 4 methods)
  - Storage container (RocksDB + Cloud + P2P)
  - Licensing container
  - Compliance container
  - Basic testing

Week_5_6_Core_Plugins:
  - Organization plugin (base features)
  - Fleet plugin (base features)
  - Operations plugin (base features)
  - Plugin registry
  - Sub-plugin system

Week_7_8_Pro_Features:
  - Organization Pro sub-plugins
  - Fleet Pro sub-plugins
  - Operations Pro sub-plugins
  - Treasury plugin (Pro)
  - Advanced testing

Week_9_10_Enterprise_Features:
  - RBAC plugin (Enterprise)
  - Multi-org plugin (Enterprise)
  - Audit extensions
  - Compliance dashboards
  - Security hardening

Week_11_12_Adapters:
  - RSI adapter (OAuth + Session + Hangar)
  - Discord adapter (OAuth + Webhooks)
  - FleetYards adapter
  - Erkul adapter
  - SCUnpacked adapter (dev-only)

Week_13_14_Integration_Testing:
  - Integration tests (500+)
  - E2E tests (50+)
  - Security testing
  - Compliance testing
  - Performance testing
  - Load testing (500 concurrent users)

Week_15_Migration_Tools:
  - V1 data export tool
  - V2 import wizard
  - Side-by-side testing
  - Data integrity verification
  - Migration documentation

Week_16_Polish_Release:
  - Bug fixes
  - Documentation completion
  - Installers (Win/Mac/Linux)
  - Code signing
  - Release notes
  - 🚀 RELEASE V2.0.0!

Post_Release:
  - V1.x maintenance (critical bugs only)
  - V1 → V2 migration support
  - V2.1 planning (based on feedback)
```

---

## ✅ 12. ACCEPTANCE CRITERIA

### 12.1 V2.0 Is Ready When

```yaml
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
# FINAL CHECKLIST
# ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Architecture:
  ✓ Core < 5MB
  ✓ Startup < 500ms
  ✓ Memory < 100MB idle
  ✓ Plugin system works
  ✓ Sub-plugins functional
  ✓ DLL hot reload works

Authentication:
  ✓ Local auth works
  ✓ Google OAuth works
  ✓ Discord OAuth works
  ✓ Twitch OAuth works
  ✓ Session management works
  ✓ License validation works

Features:
  ✓ All Free features work
  ✓ Pro features properly gated
  ✓ Enterprise features properly gated
  ✓ UI consolidated (7 main tabs)
  ✓ No bloat, no unused code

Plugins:
  ✓ Organization plugin complete (6 sub-plugins)
  ✓ Fleet plugin complete (4 sub-plugins)
  ✓ Operations plugin complete (4 sub-plugins)
  ✓ Treasury plugin complete (Pro)
  ✓ RBAC plugin complete (Enterprise)
  ✓ All adapters working (RSI, Discord, etc.)

Master_Server:
  ✓ All 10 modules implemented
  ✓ API documented
  ✓ Admin interface working
  ✓ Kill switch functional
  ✓ Update distribution works

P2P:
  ✓ Mesh networking works
  ✓ Discovery (LAN + WAN) works
  ✓ Data sync works
  ✓ Offline mode works
  ✓ Security (encryption + signing) works

Compliance:
  ✓ GDPR compliant
  ✓ CCPA compliant
  ✓ DSA compliant
  ✓ Country-specific laws implemented
  ✓ CIG ToS enforced
  ✓ Third-party ToS followed
  ✓ Audit logs tamper-proof

Security:
  ✓ Capability system enforced
  ✓ Plugin sandboxing works
  ✓ Code signing implemented
  ✓ Data signing works
  ✓ Network encryption works
  ✓ Local storage encrypted
  ✓ Kill switches functional

Testing:
  ✓ 1000+ unit tests passing
  ✓ 500+ integration tests passing
  ✓ 50+ E2E tests passing
  ✓ 95%+ code coverage
  ✓ Security audit passed
  ✓ Load tested (500 users)

Migration:
  ✓ V1 → V2 migration tool works
  ✓ Data integrity verified
  ✓ No data loss
  ✓ Side-by-side installation works
  ✓ Migration guide complete

Documentation:
  ✓ User guide complete
  ✓ Developer guide complete
  ✓ API reference complete
  ✓ Migration guide complete
  ✓ Legal documents complete

Deployment:
  ✓ Windows MSI installer
  ✓ macOS DMG installer
  ✓ Linux DEB/RPM packages
  ✓ Auto-update works
  ✓ Crash reporting active
  ✓ All code signed
  ✓ Release notes published

Performance:
  ✓ 10x faster startup than V1
  ✓ 6x less memory than V1
  ✓ 120 FPS UI capability
  ✓ Sub-5ms plugin load times
  ✓ KB-sized update downloads
```

---

## 🎯 13. FINAL DIRECTIVE

```yaml
Project_Philosophy:
  "A Cockpit, Not a Toy"
  "A System, Not a Script"
  "A Framework, Not a Shortcut"

Design_Ethos:
  - Everything shown is real
  - Everything real is visible
  - Nothing acts silently
  - Trust is paramount
  - Performance matters
  - Privacy is default
  - Compliance is built-in

Copilot_Remember:
  - This file is LAW
  - No mocks, no stubs
  - Real data only
  - Windows native
  - Offline first
  - Enterprise grade
  - Community focused

User_Experience:
  When you open Verse Guy, you should think:
    "I have control.
     Nothing happens without my knowledge.
     Nothing forces me.
     I can abort anytime.
     This is professional.
     This is mine."

That is Enterprise.
That is Community-worthy.
That is Star Citizen in spirit.
```

---

## 📖 14. DOCUMENT METADATA

```yaml
Version: 2.0.0-alpha
Status: SINGLE_SOURCE_OF_TRUTH
Authority: ABSOLUTE
Last_Updated: 2026-01-03
Author: Matthias Eckel
Review_Cycle: Weekly during development
Approval_Required: For any architecture changes

Related_Documents:
  - V1 Architecture (archived, reference only)
  - V1 → V2 Migration Guide (when created)
  - API Reference (generated from code)
  - User Manual (when created)

Change_Log:
  2026-01-03:
    - Initial V2.0 specification
    - Consolidated all previous documents
    - Integrated Windows native architecture
    - Added complete plugin-in-plugin system
    - Defined Master Server modules
    - Specified compliance framework
```

---

**END OF DOCUMENT**

This is the **SINGLE SOURCE OF TRUTH** for Verse Guy V2.0.

Any code, architecture, or design that contradicts this file is **INVALID**.

GitHub Copilot: **FOLLOW THIS DOCUMENT LITERALLY.**
