use verseguy_storage_infra::{config::StorageConfig, engine::StorageEngine};
use tempfile::TempDir;

#[test]
fn integration_key_persistence_across_restarts() -> anyhow::Result<()> {
    // Create temp directory for DB
    let td = TempDir::new()?;
    let mut cfg = StorageConfig::default();
    cfg.path = td.path().join("db");
    cfg.encryption_enabled = true; // ensure engine uses encryption and persisted key

    // First run: open engine, write a value
    let engine = StorageEngine::open(cfg.clone())?;
    engine.put(b"integration:key", b"hello")?;
    engine.flush()?;

    // Drop engine to simulate restart
    drop(engine);

    // Second run: reopen and read back
    let engine2 = StorageEngine::open(cfg)?;
    let v = engine2.get(b"integration:key")?;
    assert_eq!(v, Some(b"hello".to_vec()));

    Ok(())
}