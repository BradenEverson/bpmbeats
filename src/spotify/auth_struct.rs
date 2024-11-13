//! Serde implementation for an access token

use serde::Deserialize;

/// Client secret and ID
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ClientInfo<'a> {
    /// Client secret
    pub client_secret: &'a str,
    /// Client ID
    pub client_id: &'a str,
}

/// Access token for a client
#[derive(Deserialize, Debug, Clone, Copy)]
pub struct AccessToken<'a> {
    /// Auth token
    pub access_token: &'a str,
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
