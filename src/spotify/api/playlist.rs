//! Playlist struct response deserializing implementations

use serde::Deserialize;

/// Represents a Spotify playlist.
#[derive(Debug, Deserialize, Clone)]
pub struct Playlist {
    /// Indicates if the playlist is collaborative.
    pub collaborative: bool,
    /// Description of the playlist.
    pub description: String,
    /// External URLs related to the playlist.
    pub external_urls: ExternalUrls,
    /// Followers of the playlist.
    pub followers: Followers,
    /// API endpoint URL of the playlist.
    pub href: String,
    /// Unique identifier for the playlist.
    pub id: String,
    /// Images associated with the playlist.
    pub images: Vec<Image>,
    /// Name of the playlist.
    pub name: String,
    /// Information about the playlist owner.
    pub owner: Owner,
    /// Primary color for the playlist (if any).
    pub primary_color: Option<String>,
    /// Visibility of the playlist (public or private).
    pub public: bool,
    /// Snapshot ID for the playlist's current version.
    pub snapshot_id: String,
    /// Tracks contained in the playlist.
    pub tracks: Tracks,
    /// Type of the object, typically "playlist".
    pub r#type: String,
    /// Spotify URI of the playlist.
    pub uri: String,
}

/// Represents external URLs, typically including a Spotify link.
#[derive(Debug, Deserialize, Clone)]
pub struct ExternalUrls {
    /// Spotify URL.
    pub spotify: String,
}

/// Information about the followers of a playlist or user.
#[derive(Debug, Deserialize, Clone)]
pub struct Followers {
    /// API endpoint URL for retrieving followers (if available).
    pub href: Option<String>,
    /// Total count of followers.
    pub total: u32,
}

/// Represents an image with URL, height, and width attributes.
#[derive(Debug, Deserialize, Clone)]
pub struct Image {
    /// URL of the image.
    pub url: String,
    /// Height of the image in pixels.
    pub height: Option<u32>,
    /// Width of the image in pixels.
    pub width: Option<u32>,
}

/// Information about the owner of a playlist, typically a user.
#[derive(Debug, Deserialize, Clone)]
pub struct Owner {
    /// Display name of the owner.
    pub display_name: String,
    /// External URLs related to the owner.
    pub external_urls: ExternalUrls,
    /// API endpoint URL of the owner.
    pub href: String,
    /// Unique identifier of the owner.
    pub id: String,
    /// Type of the owner, e.g., "user".
    pub r#type: String,
    /// Spotify URI of the owner.
    pub uri: String,
}

/// Represents the list of tracks in a playlist.
#[derive(Debug, Deserialize, Clone)]
pub struct Tracks {
    /// API endpoint URL for the tracks.
    pub href: String,
    /// List of track items in the playlist.
    pub items: Vec<TrackItem>,
    /// Maximum number of tracks to return.
    pub limit: u32,
    /// URL for the next set of tracks (if available).
    pub next: Option<String>,
    /// Index of the first track in the response.
    pub offset: u32,
    /// URL for the previous set of tracks (if available).
    pub previous: Option<String>,
    /// Total number of tracks in the playlist.
    pub total: u32,
}

/// Represents an individual track item in a playlist.
#[derive(Debug, Deserialize, Clone)]
pub struct TrackItem {
    /// Date and time the track was added to the playlist.
    pub added_at: String,
    /// Information about the user who added the track.
    pub added_by: AddedBy,
    /// Whether the track is a local file.
    pub is_local: bool,
    /// Primary color associated with the track (if any).
    pub primary_color: Option<String>,
    /// Track metadata.
    pub track: Track,
    /// Thumbnail for the track's video (if any).
    pub video_thumbnail: VideoThumbnail,
}

/// Information about the user who added a track to a playlist.
#[derive(Debug, Deserialize, Clone)]
pub struct AddedBy {
    /// External URLs related to the user.
    pub external_urls: ExternalUrls,
    /// API endpoint URL of the user.
    pub href: String,
    /// Unique identifier of the user.
    pub id: String,
    /// Type of the user, e.g., "user".
    pub r#type: String,
    /// Spotify URI of the user.
    pub uri: String,
}

/// Represents a track in a playlist.
#[derive(Debug, Deserialize, Clone)]
pub struct Track {
    /// Preview URL of the track.
    pub preview_url: Option<String>,
    /// List of markets where the track is available.
    pub available_markets: Vec<String>,
    /// Whether the track has explicit content.
    pub explicit: bool,
    /// Type of the track object, e.g., "track".
    pub r#type: String,
    /// Indicates if the item is an episode.
    pub episode: bool,
    /// Indicates if the item is a track.
    pub track: bool,
    /// Album associated with the track.
    pub album: Album,
    /// List of artists for the track.
    pub artists: Vec<Artist>,
    /// Disc number of the track.
    pub disc_number: u32,
    /// Track number on the disc.
    pub track_number: u32,
    /// Duration of the track in milliseconds.
    pub duration_ms: u32,
    /// External IDs for the track.
    pub external_ids: ExternalIds,
    /// External URLs related to the track.
    pub external_urls: ExternalUrls,
    /// API endpoint URL of the track.
    pub href: String,
    /// Unique identifier of the track.
    pub id: String,
    /// Name of the track.
    pub name: String,
    /// Popularity rating of the track.
    pub popularity: u32,
    /// Spotify URI of the track.
    pub uri: String,
    /// Indicates if the track is a local file.
    pub is_local: bool,
}

/// Represents the album associated with a track.
#[derive(Debug, Deserialize, Clone)]
pub struct Album {
    /// List of markets where the album is available.
    pub available_markets: Vec<String>,
    /// Type of the album, e.g., "album".
    pub r#type: String,
    /// Album type, e.g., "album" or "single".
    pub album_type: String,
    /// API endpoint URL of the album.
    pub href: String,
    /// Unique identifier of the album.
    pub id: String,
    /// Images of the album.
    pub images: Vec<Image>,
    /// Name of the album.
    pub name: String,
    /// Release date of the album.
    pub release_date: String,
    /// Precision of the release date, e.g., "day", "month", or "year".
    pub release_date_precision: String,
    /// Spotify URI of the album.
    pub uri: String,
    /// Artists featured on the album.
    pub artists: Vec<Artist>,
    /// External URLs for the album.
    pub external_urls: ExternalUrls,
    /// Total number of tracks on the album.
    pub total_tracks: u32,
}

/// Represents an artist associated with a track or album.
#[derive(Debug, Deserialize, Clone)]
pub struct Artist {
    /// External URLs for the artist.
    pub external_urls: ExternalUrls,
    /// API endpoint URL of the artist.
    pub href: String,
    /// Unique identifier of the artist.
    pub id: String,
    /// Name of the artist.
    pub name: String,
    /// Type of the object, e.g., "artist".
    pub r#type: String,
    /// Spotify URI of the artist.
    pub uri: String,
}

/// External identifiers for a track, such as ISRC.
#[derive(Debug, Deserialize, Clone)]
pub struct ExternalIds {
    /// International Standard Recording Code.
    pub isrc: String,
}

/// Represents the video thumbnail for a track, if available.
#[derive(Debug, Deserialize, Clone)]
pub struct VideoThumbnail {
    /// URL of the video thumbnail.
    pub url: Option<String>,
}
