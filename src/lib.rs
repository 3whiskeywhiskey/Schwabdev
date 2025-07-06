//! # Schwab API Client
//!
//! A comprehensive Rust client for the Charles Schwab API, providing access to
//! trading, market data, and streaming functionality.
//!
//! ## Features
//!
//! - OAuth2 authentication with automatic token refresh
//! - Account management and trading operations
//! - Market data retrieval (quotes, option chains, price history)
//! - Real-time streaming data via WebSocket
//! - Comprehensive error handling
//! - Full async/await support
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use schwab_api::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new(
//!         "your_app_key",
//!         "your_app_secret",
//!         "https://127.0.0.1:8080"
//!     ).await?;
//!
//!     // Get account information
//!     let accounts = client.get_accounts().await?;
//!     println!("Accounts: {:?}", accounts);
//!
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod tokens;
pub mod stream;
pub mod enums;
pub mod error;
pub mod types;

pub use client::Client;
pub use error::{SchwabError, Result};
pub use enums::*;
pub use types::*;

// Re-export common types
pub use chrono::{DateTime, Utc};
pub use serde_json::Value;