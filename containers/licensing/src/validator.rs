use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Token format: base64(payload).base64(hmac)
// Payload: JSON with {"id":"...","tier":"Free|Pro|Enterprise","exp":unix_ts}

pub fn create_signed_token(payload_b64: &str, secret: &[u8]) -> Result<String> {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(payload_b64.as_bytes());
    let sig = mac.finalize().into_bytes();
    Ok(format!(
        "{}.{}",
        payload_b64,
        general_purpose::STANDARD.encode(sig)
    ))
}

pub fn validate_license(token: &str, secret: &[u8], now_unix: i64) -> Result<bool> {
    type HmacSha256 = Hmac<Sha256>;

    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 2 {
        return Ok(false);
    }
    let payload_b64 = parts[0];
    let sig_b64 = parts[1];

    let sig = general_purpose::STANDARD.decode(sig_b64)?;

    let mut mac = HmacSha256::new_from_slice(secret)?;
    mac.update(payload_b64.as_bytes());
    mac.verify_slice(&sig)?;

    // parse payload
    let payload_json = String::from_utf8(general_purpose::STANDARD.decode(payload_b64)?)?;
    let v: serde_json::Value = serde_json::from_str(&payload_json)?;
    if let Some(exp) = v.get("exp").and_then(|e| e.as_i64()) {
        return Ok(exp >= now_unix);
    }

    Ok(false)
}
