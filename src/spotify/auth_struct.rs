//! Serde implementation for an access token

use serde::Deserialize;

/// Client secret and ID
#[derive(Deserialize, Debug, Clone)]
pub struct ClientInfo<'a> {
    /// Client secret
    pub client_secret: &'a str,
    /// Client ID
    pub client_id: &'a str,
    /// Client auth refresh key
    pub refresh_token: &'a str,
    /// Client auth key
    pub auth_token: Option<String>,
}

/// Access token for a client
#[derive(Deserialize, Debug, Clone)]
pub struct AccessToken {
    /// Auth token
    pub access_token: String,
    /// Token's type
    pub token_type: TokenType,
    /// When it expires in seconds
    pub expires_in: u32,
}

/// Types of access tokens
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum TokenType {
    /// Bearer token
    Bearer,
}
