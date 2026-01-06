use ed25519_dalek as ed;
use rand::rngs::OsRng;
use rand_core::RngCore;
use ed25519_dalek::{Signer, Verifier};
use anyhow::Result;

#[derive(Clone)]
pub struct Keypair {
    signing: ed::SigningKey,
    pub public: ed::VerifyingKey,
}

impl Keypair {
    pub fn generate(rng: &mut OsRng) -> Self {
        // Generate 64 random bytes and interpret as keypair bytes
        let mut kb = [0u8; 64];
        rng.fill_bytes(&mut kb);
        let signing = ed::SigningKey::from_keypair_bytes(&kb).expect("failed to create signing key");
        let public = signing.verifying_key();
        Keypair { signing, public }
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        let mut out = [0u8; 64];
        out[..32].copy_from_slice(&self.signing.to_bytes());
        out[32..].copy_from_slice(&self.public.to_bytes());
        out
    }

    pub fn from_bytes(bytes: &[u8; 64]) -> Result<Self> {
        let mut sk = [0u8; 32];
        let mut pk = [0u8; 32];
        sk.copy_from_slice(&bytes[..32]);
        pk.copy_from_slice(&bytes[32..]);
        let signing = ed::SigningKey::from_bytes(&sk);
        let public = ed::VerifyingKey::from_bytes(&pk)?;
        Ok(Keypair { signing, public })
    }

    pub fn sign(&self, msg: &[u8]) -> Result<ed::Signature> {
        Ok(self.signing.try_sign(msg)?)
    }
}

pub type PublicKey = ed::VerifyingKey;
