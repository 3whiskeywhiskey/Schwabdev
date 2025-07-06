//! Error types for the Schwab API client

use thiserror::Error;

/// Result type alias for the Schwab API client
pub type Result<T> = std::result::Result<T, SchwabError>;

/// Main error type for the Schwab API client
#[derive(Error, Debug)]
pub enum SchwabError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Token management error
    #[error("Token error: {0}")]
    Token(String),

    /// API error response
    #[error("API error: {status} - {message}")]
    Api {
        status: u16,
        message: String,
    },

    /// Invalid parameter error
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid format
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Stream error
    #[error("Stream error: {0}")]
    Stream(String),

    /// Generic error
    #[error("Error: {0}")]
    Generic(String),
}

impl SchwabError {
    /// Create a new authentication error
    pub fn auth<T: Into<String>>(msg: T) -> Self {
        SchwabError::Auth(msg.into())
    }

    /// Create a new token error
    pub fn token<T: Into<String>>(msg: T) -> Self {
        SchwabError::Token(msg.into())
    }

    /// Create a new API error
    pub fn api<T: Into<String>>(status: u16, msg: T) -> Self {
        SchwabError::Api {
            status,
            message: msg.into(),
        }
    }

    /// Create a new configuration error
    pub fn config<T: Into<String>>(msg: T) -> Self {
        SchwabError::Config(msg.into())
    }

    /// Create a new stream error
    pub fn stream<T: Into<String>>(msg: T) -> Self {
        SchwabError::Stream(msg.into())
    }

    /// Create a new generic error
    pub fn generic<T: Into<String>>(msg: T) -> Self {
        SchwabError::Generic(msg.into())
    }
}