//! Audio Features Struct
use serde::Deserialize;

/// All Audio feature metadata
#[derive(Debug, Deserialize, Clone)]
pub struct AudioFeatures {
    /// Song's danceability
    pub danceability: f32,
    /// Energy of a song
    pub energy: f32,
    /// What key the song's in
    pub key: u32,
    /// How loud the song is
    pub loudness: f32,
    /// Mode of a song
    pub mode: u32,
    /// Speechiness
    pub speechiness: f32,
    /// How acoustic a song is
    pub acousticness: f32,
    /// The instrumentalness of a song
    pub instrumentalness: f32,
    /// The liveness of a song
    pub liveness: f32,
    /// Song's valence
    pub valence: f32,
    /// Song's tempo
    pub tempo: f32,
    /// Song's type
    pub r#type: String,
    /// Song id
    pub id: String,
    /// Song URI
    pub uri: String,
    /// Track href of song
    pub track_href: String,
    /// Analysis url of song
    pub analysis_url: String,
    /// How long the song is in milliseconds
    pub duration_ms: u32,
    /// Song's time signature
    pub time_signature: u32,
}
