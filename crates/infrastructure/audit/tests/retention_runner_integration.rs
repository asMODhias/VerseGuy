use std::process::Command;
use std::str;
use tempfile::TempDir;
use verseguy_audit_infra::{AuditEvent, AuditStore};
use verseguy_storage_infra::config::StorageConfig;
use verseguy_storage_infra::engine::StorageEngine;

#[test]
fn retention_runner_dry_run_detects_old_events() -> verseguy_storage_infra::prelude::AppResult<()> {
    // Setup DB with one old event
    let td = TempDir::new()?;
    let db_path = td.path().join("audit_runner_db");
    let cfg = StorageConfig {
        path: db_path.clone(),
        encryption_enabled: false,
        ..Default::default()
    };
    let engine = std::sync::Arc::new(StorageEngine::open(cfg)?);
    let store = AuditStore::new(engine.clone());

    let mut old = AuditEvent {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now() - chrono::Duration::days(10),
        principal_id: "dry-run-user".to_string(),
        action: "old_action".to_string(),
        resource: "res:1".to_string(),
        metadata: serde_json::Value::Object(serde_json::Map::new()),
        version: 0,
    };
    store.record(&mut old)?;

    // Drop handles so RocksDB lock is released before an external process opens the DB
    drop(store);
    drop(engine);

    // Build the retention_runner binary (ensures target exists)
    let build_status = match Command::new("cargo")
        .args([
            "build",
            "-p",
            "verseguy_audit_infra",
            "--bin",
            "retention_runner",
        ])
        .status()
    {
        Ok(s) => s,
        Err(e) => panic!("failed to run cargo build: {}", e),
    };
    assert!(build_status.success());

    // Execute the binary with --dry-run
    // Prefer the CARGO_BIN_EXE_retention_runner env var if set (Cargo provides it during `cargo test`), else fall back to target/debug path.
    // Use `cargo run` to execute the binary reliably across platforms (slower but robust in CI)
    let db_path_str = match db_path.to_str() {
        Some(s) => s,
        None => panic!("db_path is not valid UTF-8: {:?}", db_path),
    };
    let output = match Command::new("cargo")
        .args([
            "run",
            "-p",
            "verseguy_audit_infra",
            "--bin",
            "retention_runner",
            "--",
            "--db-path",
            db_path_str,
            "--days",
            "1",
            "--dry-run",
        ])
        .output()
    {
        Ok(o) => o,
        Err(e) => panic!("failed to execute cargo run for retention_runner: {}", e),
    };

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap_or_default();

    // Expect message indicating it would delete 1 event
    assert!(
        stdout.contains("Dry-run: would delete 1 events")
            || stdout.contains("would delete 1 events")
    );

    Ok(())
}
