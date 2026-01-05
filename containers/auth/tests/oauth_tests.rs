use tempfile::tempdir;

use verseguy_storage::Storage;
use verseguy_auth::{OAuthHandler};
use verseguy_auth::oauth_types::{OAuthConfig, OAuthProvider};

#[tokio::test]
async fn test_get_auth_url_providers() {
    let dir = tempdir().expect("temp dir");
    let storage = Storage::open(dir.path()).expect("open storage");

    let google_cfg = OAuthConfig::google(
        "google-client-id".to_string(),
        "google-secret".to_string(),
        "https://localhost/callback".to_string(),
    );

    let discord_cfg = OAuthConfig::discord(
        "discord-client-id".to_string(),
        "discord-secret".to_string(),
        "https://localhost/callback".to_string(),
    );

    let mut handler = OAuthHandler::new(storage);
    handler.register_provider(google_cfg);
    handler.register_provider(discord_cfg);

    let google_url = handler.get_auth_url(OAuthProvider::Google).expect("auth url");
    assert!(google_url.contains("test_client_id") || google_url.contains("google-client-id"));
    assert!(google_url.contains("redirect_uri"));

    let discord_url = handler.get_auth_url(OAuthProvider::Discord).expect("auth url");
    assert!(discord_url.contains("discord-client-id"));
    assert!(discord_url.contains("redirect_uri"));

    // Missing provider should return Err
    let res = handler.get_auth_url(OAuthProvider::Twitch);
    assert!(res.is_err());
}
