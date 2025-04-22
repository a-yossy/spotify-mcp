use anyhow::Result;
use base64::prelude::*;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

pub async fn post() -> Result<String> {
    let refresh_token = env!("SPOTIFY_REFRESH_TOKEN");
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &refresh_token),
    ];
    let client_id = env!("SPOTIFY_CLIENT_ID");
    let client_secret = env!("SPOTIFY_CLIENT_SECRET");
    let authorization = BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret));
    let client = Client::new();

    Ok(client
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", format!("Basic {}", authorization))
        .form(&params)
        .send()
        .await?
        .json::<TokenResponse>()
        .await?
        .access_token)
}
