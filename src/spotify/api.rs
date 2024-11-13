//! API Wrapper Implementations

use audio_features::AudioFeatures;
use playlist::Playlist;
use reqwest::{header::AUTHORIZATION, Client, StatusCode};

use super::auth_struct::AccessToken;

pub mod audio_features;
pub mod playlist;

/// Error type from an API call
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Reqwest error
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    /// JSON Deserialize error
    #[error("Error deserializing response")]
    JsonError(#[from] serde_json::Error),
    /// Response wasn't success
    #[error("HTTP Response failed with code {0}")]
    ResponseError(StatusCode),
}

/// Result type from API
pub type Result<T> = std::result::Result<T, Error>;

/// Spotify API session
pub struct Api<'a> {
    /// The token for the session
    token: AccessToken<'a>,
    /// The reqwest client
    client: Client,
}

impl<'a> Api<'a> {
    /// Creates a new API session from an access token
    pub fn authorize(token: AccessToken<'a>) -> Self {
        Self {
            token,
            client: Client::new(),
        }
    }

    /// Gets the audio features of a song based on its Song ID
    pub async fn get_audio_features(&self, song_id: &str) -> Result<AudioFeatures> {
        let url = format!("https://api.spotify.com/v1/audio-features/{}", song_id);
        let response_text = self.send_request(&url).await?;
        let features = serde_json::from_str(&response_text)?;
        Ok(features)
    }

    /// Gets playlist metadata from a playlist ID
    pub async fn get_playlist(&self, playlist_id: &str) -> Result<Playlist> {
        let url = format!("https://api.spotify.com/v1/playlists/{}", playlist_id);
        let response_text = self.send_request(&url).await?;
        let playlist = serde_json::from_str(&response_text)?;
        Ok(playlist)
    }

    /// Sends a request and returns it's response
    async fn send_request(&self, where_to: &str) -> Result<String> {
        let bearer_token = format!("Bearer {}", self.token.access_token);
        let response = self
            .client
            .get(where_to)
            .header(AUTHORIZATION, bearer_token)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.text().await?)
        } else {
            Err(Error::ResponseError(response.status()))
        }
    }
}
