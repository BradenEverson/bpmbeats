//! Main FW driver

use std::{
    fs::File,
    io::Read,
    sync::Arc,
    time::{Duration, Instant},
};

use bpmbeats::spotify::{
    api::Api,
    auth_struct::{AccessToken, ClientInfo},
};
use rand::{seq::SliceRandom, thread_rng};
use reqwest::Client;
use rppal::i2c::I2c;
use tokio::sync::RwLock;

/// The Playlist ID of the BPM Beats Playlist
const BPM_PLAYLIST: &'static str = "4KDw5FSSPr4UwzT0sMc1NZ";
/// Threshold for valid BPM to Tempo Songs
const THRESHOLD_SONG: f32 = 5.0;

// ADS1115 default I2C address
const ADS1115_ADDR: u16 = 0x48;

// ADS1115 registers
const CONVERSION_REG: u8 = 0x00;
const CONFIG_REG: u8 = 0x01;

// ADS1115 configuration bits
const A0_SINGLE_ENDED: u16 = 0x4000;
const FSR_4_096V: u16 = 0x0200;
const MODE_SINGLE_SHOT: u16 = 0x0100;
const DATA_RATE_128SPS: u16 = 0x0080;
const OS_START_SINGLE: u16 = 0x8000;

// Parameters for BPM calculation
const SAMPLE_INTERVAL_MS: u64 = 10;
const BPM_CALCULATION_PERIOD: Duration = Duration::from_secs(5);
const THRESHOLD: f32 = 2.7;

#[tokio::main]
async fn main() {
    let mut secrets_file = File::open("secrets.json").expect("Failed to open secrets file");

    let current_bpm = Arc::new(RwLock::new(60f32));

    // Initialize i2c for PulseSensor
    let mut i2c = I2c::new().expect("I2C Init");
    i2c.set_slave_address(ADS1115_ADDR)
        .expect("Set slave address");

    let mut beat_count = 0;
    let mut last_beat_time = Instant::now();
    let mut start_time = Instant::now();

    let bpm_clone = current_bpm.clone();
    tokio::spawn(async move {
        loop {
            let config = OS_START_SINGLE
                | A0_SINGLE_ENDED
                | FSR_4_096V
                | MODE_SINGLE_SHOT
                | DATA_RATE_128SPS;

            let config_bytes = config.to_be_bytes();
            i2c.write(&[CONFIG_REG, config_bytes[0], config_bytes[1]])
                .expect("Set config bits");

            std::thread::sleep(Duration::from_millis(10));

            let mut buffer = [0; 2];
            i2c.write_read(&[CONVERSION_REG], &mut buffer)
                .expect("Read bytes to buffer");
            let value = i16::from_be_bytes(buffer);

            let voltage = value as f32 * 4.096 / 32768.0;

            if voltage > THRESHOLD && last_beat_time.elapsed() > Duration::from_millis(300) {
                beat_count += 1;
                last_beat_time = Instant::now();
                println!("Beat detected! Voltage: {:.3} V", voltage);
            }

            if start_time.elapsed() >= BPM_CALCULATION_PERIOD {
                let bpm = (beat_count as f32 / BPM_CALCULATION_PERIOD.as_secs_f32()) * 60.0;
                *bpm_clone.write().await = bpm;

                beat_count = 0;
                last_beat_time = Instant::now();
                start_time = Instant::now();
            }

            std::thread::sleep(Duration::from_millis(SAMPLE_INTERVAL_MS));
        }
    });

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
        let current_bpm = { current_bpm.read().await.clone() };
        let viable_tracks: Vec<_> = tracks_with_tempo
            .iter()
            .filter(|(_, _, _, tempo)| (tempo - current_bpm).abs() <= THRESHOLD_SONG)
            .collect();
        if let Some((name, track, length, tempo)) = viable_tracks.choose(&mut rng) {
            println!("[Adding {name} to track] Tempo: {tempo} | Current Bpm : {current_bpm}");
            api.add_to_queue(&track).await.expect("Send song to queue");
            std::thread::sleep(Duration::from_millis(*length as u64))
        } else {
            // If BPM is not valid, sit in silence for 15 seconds
            std::thread::sleep(Duration::from_millis(15_000))
        }
    }
}

async fn refresh_access_token(
    refresh_token: &str,
    client_id: &str,
    client_secret: &str,
) -> AccessToken {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", &refresh_token),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
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
