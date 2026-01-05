use verseguy_auth::{OAuthConfig, OAuthHandler};
use verseguy_auth::oauth_types::OAuthProvider;
use verseguy_storage::Storage;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup storage (uses ./data by default)
    let storage = Storage::open("./data")?;

    let mut oauth = OAuthHandler::new(storage);

    // Register Google OAuth from environment
    let google_config = OAuthConfig::google(
        std::env::var("GOOGLE_CLIENT_ID")?,
        std::env::var("GOOGLE_CLIENT_SECRET")?,
        "http://localhost:8080/auth/callback".to_string(),
    );

    oauth.register_provider(google_config);

    // Get authorization URL
    let auth_url = oauth.get_auth_url(OAuthProvider::Google)?;
    println!("Visit this URL to authorize: {}", auth_url);

    // After user authorizes, you'll receive a callback with code and state
    // Example usage (pseudocode): oauth.handle_callback(code, state).await?

    Ok(())
}