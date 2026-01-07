use tempfile::TempDir;
use verseguy_storage_infra::{config::StorageConfig, engine::StorageEngine};

#[test]
fn test_rotate_and_reencrypt() -> anyhow::Result<()> {
    let td = TempDir::new()?;
    let cfg = StorageConfig {
        path: td.path().join("db"),
        encryption_enabled: true,
        ..Default::default()
    };

    // Open and write
    let mut engine = StorageEngine::open(cfg.clone())?;
    engine.put(b"rkey", b"secret")?;

    // Generate a new key and rotate
    let mut new_key = [0u8; 32];
    let mut rng = rand::rngs::OsRng;
    use rand::RngCore;
    rng.try_fill_bytes(&mut new_key)
        .map_err(|e| anyhow::anyhow!("rng: {}", e))?;

    engine.rotate_key_and_reencrypt(&new_key)?;

    // Read back
    let got = engine.get(b"rkey")?;
    assert_eq!(got, Some(b"secret".to_vec()));

    Ok(())
}
