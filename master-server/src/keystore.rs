use crate::ed25519_compat::Keypair;
use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine;
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

pub fn load_keypair(path: &Path) -> Result<Keypair> {
    let data = fs::read(path)?;
    if data.len() != 64 {
        anyhow::bail!("invalid key length: {}", data.len());
    }
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&data[..64]);
    let kp = Keypair::from_bytes(&arr)?;
    Ok(kp)
}

pub fn save_keypair(path: &Path, kp: &Keypair) -> Result<()> {
    let bytes = kp.to_bytes();
    fs::write(path, bytes)?;

    // Try to set restrictive permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

pub fn load_or_generate(path: &Path) -> Result<Keypair> {
    if path.exists() {
        load_keypair(path)
    } else {
        let mut csprng = OsRng {};
        let kp = Keypair::generate(&mut csprng);
        save_keypair(path, &kp)?;
        Ok(kp)
    }
}

pub fn public_key_b64_from_path(path: &Path) -> Result<String> {
    let kp = load_keypair(path)?;
    let pk = kp.public.to_bytes();
    Ok(general_purpose::STANDARD.encode(pk))
}

pub fn rotate_key(path: &Path) -> Result<Keypair> {
    // backup old if exists
    if path.exists() {
        let bak = path.with_extension("bak");
        std::fs::copy(path, &bak)?;
    }
    let mut csprng = OsRng {};
    let kp = Keypair::generate(&mut csprng);
    save_keypair(path, &kp)?;
    Ok(kp)
}

pub fn import_key_b64(path: &Path, b64: &str) -> Result<Keypair> {
    let bytes = general_purpose::STANDARD.decode(b64)?;
    if bytes.len() != 64 {
        anyhow::bail!("invalid key length: {}", bytes.len());
    }
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&bytes[..64]);
    let kp = Keypair::from_bytes(&arr)?;
    save_keypair(path, &kp)?;
    Ok(kp)
}
