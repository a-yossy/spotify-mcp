use anyhow::Result;
use reqwest::Client;

type ContainsResponse = Vec<bool>;
pub enum Type {
    Artist,
    User,
}

pub async fn get(access_token: &str, r#type: Type, ids: &[String]) -> Result<ContainsResponse> {
    let client = Client::new();
    let response = client
        .get("https://api.spotify.com/v1/me/following/contains")
        .query(&[
            (
                "type",
                match r#type {
                    Type::Artist => "artist",
                    Type::User => "user",
                },
            ),
            ("ids", &ids.join(",")),
        ])
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<ContainsResponse>()
        .await?;

    Ok(response)
}
