use anyhow::{Context, Result};
use reqwest::Client;
use tracing::{info};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use url::Url;

use crate::types::{AuthMethod, License, User};
use crate::oauth_types::{OAuthConfig, OAuthProvider, OAuthState, TokenResponse, OAuthUserInfo};
use verseguy_storage::Storage;

/// OAuth handler implementing Provider configs and state management
pub struct OAuthHandler {
    storage: Storage,
    client: Client,
    configs: HashMap<OAuthProvider, OAuthConfig>,
    states: Arc<RwLock<HashMap<String, OAuthState>>>,
}

impl OAuthHandler {
    /// Create new OAuth handler
    pub fn new(storage: Storage) -> Self {
        Self {
            storage,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to build HTTP client"),
            configs: HashMap::new(),
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register OAuth provider config
    pub fn register_provider(&mut self, config: OAuthConfig) {
        info!("Registering OAuth provider: {:?}", config.provider);
        self.configs.insert(config.provider, config);
    }

    /// Generate authorization URL and store state for CSRF protection
    pub fn get_auth_url(&self, provider: OAuthProvider) -> Result<String> {
        let cfg = self
            .configs
            .get(&provider)
            .ok_or_else(|| anyhow::anyhow!("Unsupported provider"))?;

        let state = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let s = OAuthState { state: state.clone(), provider, created_at: now };
        self.states.write().expect("oauth states RwLock poisoned").insert(state.clone(), s);

        // Build auth URL
        let mut url = Url::parse(&cfg.auth_url).context("Invalid auth url")?;
        url.query_pairs_mut()
            .append_pair("client_id", &cfg.client_id)
            .append_pair("redirect_uri", &cfg.redirect_uri)
            .append_pair("response_type", "code")
            .append_pair("state", &state);

        // Default scopes
        match provider {
            OAuthProvider::Google => {
                url.query_pairs_mut().append_pair("scope", "openid email profile");
            }
            OAuthProvider::Discord => {
                url.query_pairs_mut().append_pair("scope", "identify email");
            }
            OAuthProvider::Twitch => {
                url.query_pairs_mut().append_pair("scope", "user:read:email");
            }
        }

        Ok(url.to_string())
    }

    /// Handle OAuth callback: validate state, exchange code, create or return user
    pub async fn handle_callback(&self, code: String, state: String) -> Result<User> {
        // Verify state (CSRF protection)
        let oauth_state = {
            let mut states = self.states.write().expect("oauth states RwLock poisoned");
            states.remove(&state)
                .ok_or_else(|| anyhow::anyhow!("Invalid or expired state"))?
        };

        // Check state age (max 10 minutes)
        let age = chrono::Utc::now() - oauth_state.created_at;
        if age > chrono::Duration::minutes(10) {
            anyhow::bail!("State expired");
        }

        let provider = oauth_state.provider;

        // Exchange code for token
        let token = self.exchange_code(provider, code).await?;

        // Get user info
        let user_info = self.get_user_info(provider, &token.access_token).await?;

        // Check existing mapping
        let email_key = format!("user_by_oauth:{}:{}", provider.as_str(), user_info.id);
        let existing_user_id: Option<String> = self
            .storage
            .get(email_key.as_bytes())
            .context("Failed to check existing OAuth user")?;

        if let Some(user_id) = existing_user_id {
            return self
                .storage
                .get(user_id.as_bytes())
                .context("Failed to get user")?
                .ok_or_else(|| anyhow::anyhow!("User not found"));
        }

        let user_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::seconds(token.expires_in);

        let user = User {
            id: user_id.clone(),
            username: user_info.name.clone().unwrap_or_else(|| format!("user_{}", &user_id[..8])),
            email: user_info.email.clone(),
            password_hash: None,
            auth_method: AuthMethod::OAuth {
                provider: provider.as_str().to_string(),
                token: token.access_token.clone(),
                refresh_token: token.refresh_token.clone(),
                expires_at,
            },

            license: License::Pro,
            created_at: now,
            updated_at: now,
        };

        self.storage.put(user_id.as_bytes(), &user).context("Failed to save user")?;
        self.storage.put(email_key.as_bytes(), &user_id).context("Failed to save OAuth mapping")?;

        Ok(user)
    }

    async fn exchange_code(&self, provider: OAuthProvider, code: String) -> Result<TokenResponse> {
        let config = self.configs.get(&provider)
            .ok_or_else(|| anyhow::anyhow!("Provider not configured"))?;

        let params = [
            ("client_id", config.client_id.clone()),
            ("client_secret", config.client_secret.clone()),
            ("code", code),
            ("grant_type", "authorization_code".to_string()),
            ("redirect_uri", config.redirect_uri.clone()),
        ];

        let resp = self
            .client
            .post(&config.token_url)
            .form(&params)
            .send()
            .await
            .context("Failed to send token request")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Token exchange failed: {} - {}", status, body);
        }

        let token: TokenResponse = resp
            .json()
            .await
            .context("Failed to parse token response")?;

        Ok(token)
    }

    pub async fn refresh_token(&self, cfg: &OAuthConfig, refresh_token: &str) -> Result<TokenResponse> {
        let params = [
            ("client_id", cfg.client_id.clone()),
            ("client_secret", cfg.client_secret.clone()),
            ("refresh_token", refresh_token.to_string()),
            ("grant_type", "refresh_token".to_string()),
        ];

        let resp = self
            .client
            .post(&cfg.token_url)
            .form(&params)
            .send()
            .await
            .context("Failed to refresh token")?;

        if !resp.status().is_success() {
            anyhow::bail!("Token refresh failed: {}", resp.status());
        }

        let token = resp.json::<TokenResponse>().await.context("Failed to parse token response")?;
        Ok(token)
    }

    async fn get_user_info(&self, provider: OAuthProvider, access_token: &str) -> Result<OAuthUserInfo> {
        let cfg = self.configs.get(&provider).ok_or_else(|| anyhow::anyhow!("Provider not configured"))?;
        let resp = self
            .client
            .get(&cfg.userinfo_url)
            .bearer_auth(access_token)
            .send()
            .await
            .context("Failed to get user info")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("Get user info failed: {} - {}", status, body);
        }

        let user_info: OAuthUserInfo = resp
            .json()
            .await
            .context("Failed to parse user info")?;
        Ok(user_info)
    }
}
