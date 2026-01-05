#![cfg(feature = "oauth_integration")]

use mockito::Server;
use tempfile::tempdir;
use verseguy_storage::Storage;
use verseguy_auth::{OAuthHandler};
use verseguy_auth::oauth_types::{OAuthConfig, OAuthProvider};

#[tokio::test]
async fn test_oauth_flow_with_mock() {
    let mut server = tokio::task::spawn_blocking(|| Server::new())
        .await
        .expect("failed to create mock server");

    let _m1 = server.mock("POST", "/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "access_token": "mock_at", "refresh_token": "mock_rt", "expires_in": 3600, "token_type": "Bearer" }"#)
        .create();

    let _m2 = server.mock("GET", "/userinfo")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "id": "user123", "email": "test@example.com", "name": "Test User" }"#)
        .create();

    let cfg = OAuthConfig {
        provider: OAuthProvider::Google,
        client_id: "cid".to_string(),
        client_secret: "csecret".to_string(),
        redirect_uri: "http://localhost/callback".to_string(),
        auth_url: format!("{}/auth", server.url()),
        token_url: format!("{}/token", server.url()),
        userinfo_url: format!("{}/userinfo", server.url()),
    };

    let dir = tempdir().unwrap();
    let storage = Storage::open(dir.path()).unwrap();

    let mut handler = OAuthHandler::new(storage);
    handler.register_provider(cfg);

    // get url and extract state
    let url = handler.get_auth_url(OAuthProvider::Google).unwrap();
    let parsed = url::Url::parse(&url).unwrap();
    let state = parsed.query_pairs().find(|(k,_)| k=="state").map(|(_,v)| v.to_string()).unwrap();

    // simulate callback with code (code first, state second)
    let user = handler.handle_callback("fakecode".to_string(), state).await.unwrap();

    assert_eq!(user.email.unwrap(), "test@example.com");
}
