use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use verseguy_storage::RocksDBStorage;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<i64>,
}

impl PluginManifest {
    pub fn with_published(&self) -> Self {
        let mut m = self.clone();
        m.published_at = Some(Utc::now().timestamp());
        m
    }
}

fn canonical_manifest_bytes(manifest: &PluginManifest) -> Result<Vec<u8>> {
    // Serialize deterministically
    let bytes = serde_json::to_vec(manifest)?;
    Ok(bytes)
}

pub fn store_manifest(
    storage: &RocksDBStorage,
    manifest: &PluginManifest,
    keypair: Option<&Keypair>,
) -> Result<()> {
    let key = format!("plugin:{}:{}", manifest.id, manifest.version);
    storage.put(key.as_bytes(), manifest)?;

    if let Some(kp) = keypair {
        let bytes = canonical_manifest_bytes(manifest)?;
        let sig: Signature = kp.sign(&bytes);
        let sig_b64 = general_purpose::STANDARD.encode(sig.to_bytes());
        let sig_key = format!("plugin_sig:{}:{}", manifest.id, manifest.version);
        storage.put(sig_key.as_bytes(), &sig_b64)?;
    }

    Ok(())
}

pub fn search_manifests(storage: &RocksDBStorage, q: &str) -> Result<Vec<PluginManifest>> {
    let mut results = Vec::new();
    let items: Vec<PluginManifest> = storage.prefix_scan(b"plugin:")?;
    let q_lower = q.to_lowercase();
    for it in items {
        if it.name.to_lowercase().contains(&q_lower) || it.id.to_lowercase().contains(&q_lower) {
            results.push(it);
        }
    }
    Ok(results)
}

pub fn verify_manifest(
    storage: &RocksDBStorage,
    manifest: &PluginManifest,
    pubkey: &PublicKey,
) -> Result<bool> {
    let bytes = canonical_manifest_bytes(manifest)?;
    let sig_key = format!("plugin_sig:{}:{}", manifest.id, manifest.version);
    let sig_b64: Option<String> = storage.get(sig_key.as_bytes())?;
    let sig_b64 = sig_b64.ok_or_else(|| anyhow::anyhow!("signature not found"))?;
    let sig_bytes = general_purpose::STANDARD.decode(sig_b64)?;
    let sig = Signature::from_bytes(&sig_bytes)?;
    match pubkey.verify(&bytes, &sig) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn revoke_manifest(
    storage: &RocksDBStorage,
    id: &str,
    version: &str,
    reason: &str,
) -> Result<()> {
    let key = format!("plugin_revoked:{}:{}", id, version);
    let entry = serde_json::json!({"reason": reason, "at": Utc::now().timestamp()});
    storage.put(key.as_bytes(), &entry)?;
    Ok(())
}

pub fn is_revoked(storage: &RocksDBStorage, id: &str, version: &str) -> Result<bool> {
    let key = format!("plugin_revoked:{}:{}", id, version);
    let v: Option<serde_json::Value> = storage.get(key.as_bytes())?;
    Ok(v.is_some())
}
