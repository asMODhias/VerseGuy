#![allow(clippy::disallowed_methods)]
use tempfile::tempdir;

use verseguy_storage::Storage;
use verseguy_auth::{OAuthHandler};
use verseguy_auth::oauth_types::{OAuthConfig, OAuthProvider};
use verseguy_test_utils::must;

#[tokio::test]
async fn test_get_auth_url_providers() {
    let dir = must(tempdir());
    let storage = must(Storage::open(dir.path()));

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

    let google_url = must(handler.get_auth_url(OAuthProvider::Google));
    assert!(google_url.contains("test_client_id") || google_url.contains("google-client-id"));
    assert!(google_url.contains("redirect_uri"));

    let discord_url = must(handler.get_auth_url(OAuthProvider::Discord));
    assert!(discord_url.contains("discord-client-id"));
    assert!(discord_url.contains("redirect_uri"));

    // Missing provider should return Err
    let res = handler.get_auth_url(OAuthProvider::Twitch);
    match res {
        Err(_) => (),
        Ok(_) => panic!("expected Err for missing provider"),
    }
}
