# Startup Sequence — Extract

## Target Startup Timings
- Time_0ms: Launch `VerseguY.exe` stub
- Time_50ms: Load `VerseguY.Core.dll`
- Time_100ms: WinUI 3 window appears
- Time_200ms: Discover plugins, init registries
- Time_300ms: Decide onboarding vs login
- Time_500ms: UI fully loaded (ready)

## First-run Logic (Rust snippet)
```rust
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
```

Notes:
- Core must be minimal (<5MB) and avoid business logic.
- Plugin discovery must be fast; plugin load times target ~5ms per plugin.

---
Source: `VERSE_GUY_V2_ULTIMATE_SPECIFICATION.md`