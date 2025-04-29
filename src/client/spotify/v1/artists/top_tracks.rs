use crate::constant::spotify::API_BASE_URL;
use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TopTracksResponse {
    tracks: Vec<Track>,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub external_urls: ExternalUrls,
}

#[derive(Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

pub async fn get(artist_id: &str, access_token: &str) -> Result<Vec<Track>> {
    let mut tracks = Vec::new();
    let client = Client::new();
    let response = client
        .get(&format!(
            "{}/v1/artists/{}/top-tracks",
            API_BASE_URL, artist_id
        ))
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<TopTracksResponse>()
        .await?;
    response
        .tracks
        .into_iter()
        .for_each(|track| tracks.push(track));

    Ok(tracks)
}
