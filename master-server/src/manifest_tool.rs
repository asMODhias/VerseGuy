use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use std::fs;

pub fn canonical_bytes_from_path(path: &str) -> Result<Vec<u8>> {
    let data = fs::read_to_string(path)?;
    let v: serde_json::Value = serde_json::from_str(&data)?;
    let bytes = serde_json::to_vec(&v)?;
    Ok(bytes)
}

pub fn sign_manifest(manifest: &str, out_sig: &str, out_key: &str, out_pub: &str) -> Result<()> {
    println!("manifest-tool: signing manifest: {}", manifest);
    let bytes = canonical_bytes_from_path(manifest)?;
    let mut csprng = OsRng {};
    let kp: Keypair = Keypair::generate(&mut csprng);
    let sig: Signature = kp.sign(&bytes);
    fs::write(out_sig, general_purpose::STANDARD.encode(sig.to_bytes()))?;
    fs::write(out_key, kp.to_bytes())?;
    fs::write(
        out_pub,
        general_purpose::STANDARD.encode(kp.public.to_bytes()),
    )?;
    println!(
        "manifest-tool: wrote sig={} key={} pub={}",
        out_sig, out_key, out_pub
    );
    Ok(())
}

pub fn verify_manifest(manifest: &str, sigfile: &str, pubfile: &str) -> Result<bool> {
    println!(
        "manifest-tool: verifying manifest: {} (sig={}, pub={})",
        manifest, sigfile, pubfile
    );
    let bytes = canonical_bytes_from_path(manifest)?;
    let sig_b64 = fs::read_to_string(sigfile)?;
    let sig_bytes = general_purpose::STANDARD.decode(sig_b64.trim())?;
    let sig = Signature::from_bytes(&sig_bytes)?;
    let pub_b64 = fs::read_to_string(pubfile)?;
    let pub_bytes = general_purpose::STANDARD.decode(pub_b64.trim())?;
    let pubk = PublicKey::from_bytes(&pub_bytes)?;
    match pubk.verify(&bytes, &sig) {
        Ok(_) => {
            println!("manifest-tool: verification OK");
            Ok(true)
        }
        Err(_) => {
            println!("manifest-tool: verification FAILED");
            Ok(false)
        }
    }
}
