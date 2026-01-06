use anyhow::Result;

#[test]
fn test_store_and_load_keyring_roundtrip() -> Result<()> {
    // Use a unique service/account name to avoid clobbering real entries
    let service = format!("verseguy-test-{}", std::process::id());
    let account = "unittest";

    let key = [42u8; 32];

    // Try to store
    verseguy_storage::secrets::store_encryption_key(&service, account, &key)?;

    // Load back
    let loaded = verseguy_storage::secrets::load_encryption_key(&service, account)?;
    assert_eq!(loaded, Some(key.to_vec()));

    // Attempt to clean up (best effort)
    let _ = keyring::Entry::new(&service, account).delete_password();

    Ok(())
}
