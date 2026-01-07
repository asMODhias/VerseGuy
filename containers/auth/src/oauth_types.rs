use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// OAuth provider enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Google,
    Discord,
    Twitch,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "google",
            OAuthProvider::Discord => "discord",
            OAuthProvider::Twitch => "twitch",
        }
    }
}

/// OAuth provider configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub provider: OAuthProvider,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: String,
}

impl OAuthConfig {
    pub fn google(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            provider: OAuthProvider::Google,
            client_id,
            client_secret,
            redirect_uri,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
        }
    }

    pub fn discord(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            provider: OAuthProvider::Discord,
            client_id,
            client_secret,
            redirect_uri,
            auth_url: "https://discord.com/api/oauth2/authorize".to_string(),
            token_url: "https://discord.com/api/oauth2/token".to_string(),
            userinfo_url: "https://discord.com/api/users/@me".to_string(),
        }
    }

    pub fn twitch(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            provider: OAuthProvider::Twitch,
            client_id,
            client_secret,
            redirect_uri,
            auth_url: "https://id.twitch.tv/oauth2/authorize".to_string(),
            token_url: "https://id.twitch.tv/oauth2/token".to_string(),
            userinfo_url: "https://api.twitch.tv/helix/users".to_string(),
        }
    }
}

/// OAuth state (CSRF protection)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthState {
    pub state: String,
    pub provider: OAuthProvider,
    pub created_at: DateTime<Utc>,
}

/// Token response
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
}

/// User info from OAuth provider
#[derive(Debug, Deserialize)]
pub struct OAuthUserInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub username: Option<String>,
}
