use crate::License;
use anyhow::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use verseguy_storage::RocksDBStorage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionClaims {
    pub sub: String,
    pub exp: i64,
    pub license: String,
    pub sid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionRecord {
    pub sid: String,
    pub user_id: String,
    pub license: String,
    pub created_at: i64,
    pub expires_at: i64,
}

pub struct SessionService {
    secret: Vec<u8>,
}

impl SessionService {
    pub fn new(secret: Vec<u8>) -> Self {
        Self { secret }
    }

    /// Create a JWT and persist a SessionRecord in RocksDB under key `session:{sid}`
    pub fn create_and_store_session(
        &self,
        user_id: &str,
        license: &License,
        days_valid: i64,
        storage: &RocksDBStorage,
    ) -> Result<String> {
        let created = Utc::now();
        let exp = created + Duration::days(days_valid);
        let sid = Uuid::new_v4().to_string();

        let claims = SessionClaims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            license: format!("{:?}", license),
            sid: sid.clone(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )?;

        let rec = SessionRecord {
            sid: sid.clone(),
            user_id: user_id.to_string(),
            license: claims.license.clone(),
            created_at: created.timestamp(),
            expires_at: exp.timestamp(),
        };

        let key = format!("session:{}", sid);
        storage.put(key.as_bytes(), &rec)?;

        Ok(token)
    }

    /// Validate JWT and ensure corresponding session record exists and is not expired
    pub fn validate_token_and_storage(
        &self,
        token: &str,
        storage: &RocksDBStorage,
    ) -> Result<TokenData<SessionClaims>> {
        let data = decode::<SessionClaims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &Validation::default(),
        )?;
        // Check expiry in token
        if data.claims.exp < Utc::now().timestamp() {
            anyhow::bail!("Session expired");
        }

        // Check storage
        let key = format!("session:{}", data.claims.sid);
        let rec_opt: Option<SessionRecord> = storage.get(key.as_bytes())?;
        let rec = rec_opt.ok_or_else(|| anyhow::anyhow!("Session not found in storage"))?;

        if rec.expires_at < Utc::now().timestamp() {
            anyhow::bail!("Session expired in storage");
        }

        Ok(data)
    }
}
