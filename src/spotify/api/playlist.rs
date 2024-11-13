//! Playlist struct response deserializing implementations

use serde::Deserialize;

/// A playlist and all of its metadata
#[derive(Deserialize, Debug, Clone)]
pub struct Playlist {
    /// Is the playlist collaborative?
    pub collaborative: bool,
    /// Playlist description
    pub description: String,
}
