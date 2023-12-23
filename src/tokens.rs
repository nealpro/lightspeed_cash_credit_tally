use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Structure for the refresh token request
#[derive(Serialize)]
struct RefreshTokenRequest {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    grant_type: String,
}

// Structure for the access token response
#[derive(Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    expires_in: u64,
    token_type: String,
    scope: String,
}

pub async fn get_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://cloud.lightspeedapp.com/oauth/access_token.php")
        .json(&RefreshTokenRequest {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            refresh_token: refresh_token.to_string(),
            grant_type: "refresh_token".to_string(),
        })
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;

    println!(
        "Access token expires in: {}\nToken type: {}\nScope: {}",
        response.expires_in, response.token_type, response.scope
    );

    Ok(response.access_token)
}
