//! Main FW driver

use std::{fs::File, io::Read};

use bpmbeats::spotify::{
    api::Api,
    auth_struct::{AccessToken, ClientInfo},
};
use reqwest::{header::CONTENT_TYPE, Client};

#[tokio::main]
async fn main() {
    let client = Client::new();
    let url = "https://accounts.spotify.com/api/token";

    let mut secrets_file = File::open("secrets.json").expect("Failed to open secrets file");
    let mut buf = String::new();
    secrets_file
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let client_info: ClientInfo<'_> =
        serde_json::from_str(&buf).expect("Failed to deserialize secrets");

    let params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_info.client_id),
        ("client_secret", client_info.client_secret),
    ];

    let response = client
        .post(url)
        .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .expect("Failed to get response");

    let response = if response.status().is_success() {
        response.text().await.expect("Failed to parse text")
    } else {
        panic!("Failed to fetch token: {}", response.status());
    };

    let access_token: AccessToken<'_> =
        serde_json::from_str(&response).expect("Failed to deserialize access token");
    let api = Api::authorize(access_token);

    /// Tyler the Creator's song "Like Him"
    const LIKE_HIM: &str = "6jbYpRPTEFl1HFKHk1IC0m";

    let song = api
        .get_audio_features(LIKE_HIM)
        .await
        .expect("Failed to get song info");

    println!("{:?}", song);

    api.get_playlist("3cEYpjA9oz9GiPac4AsH4n")
        .await
        .expect("Get random playlist");
}
