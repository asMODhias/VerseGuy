use anyhow::Result;
use ed25519_dalek as ed;
use ed25519_dalek::Signer;
use rand::rngs::OsRng;
use rand::RngCore;

#[derive(Clone)]
pub struct Keypair {
    signing: ed::SigningKey,
    pub public: ed::VerifyingKey,
}

impl Keypair {
    pub fn generate(rng: &mut OsRng) -> Self {
        // Generate a secret key and derive the public key
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut sk);
        let signing = ed::SigningKey::from_bytes(&sk);
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
