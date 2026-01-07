use anyhow::Result;
use mockito::{mock, server_url};
use tempfile::TempDir;
use std::sync::Arc;
use verseguy_auth::prelude::*;
use verseguy_storage_infra::config::StorageConfig;
use verseguy_storage_infra::engine::StorageEngine;

#[test]
fn auth_storage_and_session_flow() -> Result<()> {
    let td = TempDir::new()?;
    let cfg = StorageConfig { path: td.path().join("db"), encryption_enabled: false, ..Default::default() };
    let engine = Arc::new(StorageEngine::open(cfg)?);

    let repo = user::repo::StorageUserRepository::new(engine.clone());
    let store = session::store::StorageSessionStore::new(engine.clone());

    let mut user = user::User {
        id: uuid::Uuid::new_v4().to_string(),
        username: "integration_user".to_string(),
        email: "int@example.com".to_string(),
        password_hash: "hash".to_string(),
        version: 0,
    };

    repo.create_user(&mut user)?;

    let loaded = repo.get_user_by_id(&user.id)?.expect("user created");
    assert_eq!(loaded.username, "integration_user");

    // create session
    let mut s = session::Session {
        id: uuid::Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        created: chrono::Utc::now(),
        expires: chrono::Utc::now(),
        version: 0,
    };

    store.create_session(&mut s)?;
    let got = store.get_session(&s.id)?.expect("session exists");
    assert_eq!(got.user_id, s.user_id);

    store.revoke_session(&s.id)?;
    assert!(store.get_session(&s.id)?.is_none());

    repo.delete_user(&user.id)?;
    assert!(repo.get_user_by_id(&user.id)?.is_none());

    Ok(())
}

#[test]
fn oauth_token_exchange_mocked() -> Result<()> {
    // Mock token endpoint returning access_token JSON as oauth2 expects
    let _m = mock("POST", "/token")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{ "access_token": "mocked-token", "token_type": "Bearer", "expires_in": 3600 }"#)
        .create();

    let auth_url = url::Url::parse(&format!("{}/auth", server_url()))?;
    let token_url = url::Url::parse(&format!("{}/token", server_url()))?;

    let client = oauth2::basic::BasicClient::new(
        oauth2::ClientId::new("id".to_string()),
        Some(oauth2::ClientSecret::new("secret".to_string())),
        auth_url,
        Some(token_url),
    );

    let provider = oauth::provider::OAuthClient::new(client);
    let token = provider.exchange_code("code").expect("token exchange");

    assert!(token.contains("mocked-token") || token == "mocked-token");
    Ok(())
}