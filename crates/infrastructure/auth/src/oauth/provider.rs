use oauth2::{basic::BasicClient, AuthorizationCode};

pub struct OAuthClient {
    client: BasicClient,
}

impl OAuthClient {
    pub fn new(client: BasicClient) -> Self {
        Self { client }
    }
}

impl super::OAuthProvider for OAuthClient {
    fn exchange_code(&self, code: &str) -> anyhow::Result<String> {
        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request(oauth2::reqwest::http_client)
            .map_err(|e| anyhow::anyhow!("OAuth token exchange failed: {}", e))?;

        Ok(token.access_token().secret().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl};

    #[test]
    fn client_constructs() {
        let client = BasicClient::new(
            ClientId::new("id".to_string()),
            Some(ClientSecret::new("secret".to_string())),
            AuthUrl::new("https://example.com/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://example.com/token".to_string()).unwrap()),
        );

        let c = OAuthClient::new(client);
        // We can't call exchange_code without a real endpoint here; ensure type compiles
        let _ = c;
    }
}