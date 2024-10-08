use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    // expires_in: u64,
    // scope: String,
}

#[derive(Clone)]
pub struct TokenClient {
    client: ReqwestClient,
    client_id: String,
    client_secret: String,
    token_url: String,
}

impl TokenClient {
    pub fn new() -> Self {
        let client_id = env::var("ELATION_CLIENT_ID").expect("ELATION_CLIENT_ID must be set");
        let client_secret =
            env::var("ELATION_CLIENT_SECRET").expect("ELATION_CLIENT_SECRET must be set");
        let api_url = env::var("ELATION_API_URL").expect("Expected ELATION_API_URL to be set!");
        let token_url = format!("{api_url}/oauth2/token/");

        TokenClient {
            client: ReqwestClient::new(),
            client_id,
            client_secret,
            token_url,
        }
    }

    pub async fn fetch_token(&self) -> Result<String, reqwest::Error> {
        let mut params = HashMap::new();

        params.insert("grant_type", "client_credentials");
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);

        let response = self
            .client
            .post(&self.token_url)
            .form(&params)
            .send()
            .await?
            .json::<TokenResponse>()
            .await?;

        Ok(response.access_token)
    }
}
