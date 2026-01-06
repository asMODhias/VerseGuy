use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use tracing::{info, warn};

/// System keyring integration for storing sensitive secrets like the
/// encryption key. Uses the `keyring` crate which delegates to the
/// platform's native credential store (Windows Credential Manager, macOS Keychain,
/// or libsecret on Linux).
pub fn store_encryption_key(service: &str, account: &str, key: &[u8]) -> Result<()> {
    // Encode as base64 to ensure safe storage as UTF-8 string
    let encoded = general_purpose::STANDARD.encode(key);

    // Use Entry API from `keyring` crate
    let entry = keyring::Entry::new(service, account);

    match entry.set_password(&encoded) {
        Ok(()) => {
            info!(%service, %account, "Stored encryption key in system keyring");
            Ok(())
        }
        Err(e) => {
            // Do not panic — surface an error to the caller
            warn!(%service, %account, error = %e, "Failed to store key in keyring");
            Err(anyhow::anyhow!(
                "Failed to store key in system keyring: {}",
                e
            ))
        }
    }
}

pub fn load_encryption_key(service: &str, account: &str) -> Result<Option<Vec<u8>>> {
    let entry = keyring::Entry::new(service, account);

    match entry.get_password() {
        Ok(pw) => {
            let decoded = general_purpose::STANDARD
                .decode(&pw)
                .context("Failed to decode base64 encryption key from keyring")?;
            Ok(Some(decoded))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => {
            warn!(%service, %account, error = %e, "Failed to load key from keyring");
            Err(anyhow::anyhow!(
                "Failed to load encryption key from system keyring: {}",
                e
            ))
        }
    }
}
