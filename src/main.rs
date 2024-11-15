//! Main FW driver

use std::{fs::File, io::Read, sync::Arc, time::Duration};

use bpmbeats::{
    pulse_sensor::PulseSensor,
    spotify::{
        api::Api,
        auth_struct::{AccessToken, ClientInfo},
    },
};
use rand::{seq::SliceRandom, thread_rng};
use reqwest::Client;
use tokio::sync::RwLock;

/// The Playlist ID of the BPM Beats Playlist
const BPM_PLAYLIST: &str = "4KDw5FSSPr4UwzT0sMc1NZ";
/// Threshold for valid BPM to Tempo Songs
const THRESHOLD: f32 = 5.0;

#[tokio::main]
async fn main() {
    let mut secrets_file = File::open("secrets.json").expect("Failed to open secrets file");

    let current_bpm = Arc::new(RwLock::new(60f32));

    let pulse_sensor = PulseSensor::new(current_bpm.clone());
    tokio::spawn(async move { pulse_sensor.run().await });

    let mut buf = String::new();
    secrets_file
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let client_info: ClientInfo<'_> =
        serde_json::from_str(&buf).expect("Failed to deserialize secrets");

    let access_token = refresh_access_token(
        client_info.refresh_token,
        client_info.client_id,
        client_info.client_secret,
    )
    .await;

    let api = Api::authorize(access_token);

    let playlist = api
        .get_playlist(BPM_PLAYLIST)
        .await
        .expect("Get random playlist");

    let tracks: Vec<_> = playlist
        .tracks
        .items
        .into_iter()
        .map(|song| (song.track.name, song.track.id, song.track.duration_ms))
        .collect();

    let mut tracks_with_tempo = vec![];

    for (name, track, length) in tracks {
        let tempo = api
            .get_audio_features(&track)
            .await
            .expect("Failed to get song features")
            .tempo;

        tracks_with_tempo.push((name, track, length, tempo))
    }

    let mut rng = thread_rng();

    loop {
        let current_bpm = { *current_bpm.read().await };
        let viable_tracks: Vec<_> = tracks_with_tempo
            .iter()
            .filter(|(_, _, _, tempo)| (tempo - current_bpm).abs() <= THRESHOLD)
            .collect();
        if let Some((name, track, length, tempo)) = viable_tracks.choose(&mut rng) {
            println!("[Adding {name} to track] Tempo: {tempo} | Current Bpm : {current_bpm}");
            api.add_to_queue(track).await.expect("Send song to queue");
            std::thread::sleep(Duration::from_millis(*length as u64))
        } else {
            // If BPM is not valid, sit in silence for 15 seconds
            println!("No songs close to Bpm {current_bpm} :( waiting 15 seconds");
            std::thread::sleep(Duration::from_millis(15_000))
        }
    }
}

/// Refreshes an access token with permissions to queue up music
async fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
    client_secret: &str,
) -> AccessToken {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];

    // Send the POST request to Spotify's API
    let client = Client::new();
    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let response_text = response.text().await.expect("Failed to get response text");
        let access_token: AccessToken =
            serde_json::from_str(&response_text).expect("Failed to deserialize access token");

        access_token
    } else {
        panic!("Failed to refresh token");
    }
}
