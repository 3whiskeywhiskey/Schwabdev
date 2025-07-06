//! Token management for the Schwab API client

use crate::error::{Result, SchwabError};
use crate::types::{SavedTokens, TokenResponse};
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use log::{info, warn, error};
use reqwest::Client as HttpClient;
use serde_json;
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration as TokioDuration;

/// Token manager for handling OAuth2 authentication
pub struct TokenManager {
    app_key: String,
    app_secret: String,
    callback_url: String,
    tokens_file: String,
    http_client: HttpClient,
    
    // Token data
    access_token: Arc<RwLock<Option<String>>>,
    refresh_token: Arc<RwLock<Option<String>>>,
    id_token: Arc<RwLock<Option<String>>>,
    access_token_issued: Arc<RwLock<DateTime<Utc>>>,
    refresh_token_issued: Arc<RwLock<DateTime<Utc>>>,
    
    // Token timeouts (in seconds)
    access_token_timeout: i64,
    refresh_token_timeout: i64,
}

impl TokenManager {
    /// Create a new token manager
    pub fn new(
        app_key: String,
        app_secret: String,
        callback_url: String,
        tokens_file: Option<String>,
    ) -> Result<Self> {
        // Validate input parameters
        Self::validate_input(&app_key, &app_secret, &callback_url)?;
        
        let tokens_file = tokens_file.unwrap_or_else(|| "tokens.json".to_string());
        
        Ok(TokenManager {
            app_key,
            app_secret,
            callback_url,
            tokens_file,
            http_client: HttpClient::new(),
            access_token: Arc::new(RwLock::new(None)),
            refresh_token: Arc::new(RwLock::new(None)),
            id_token: Arc::new(RwLock::new(None)),
            access_token_issued: Arc::new(RwLock::new(DateTime::from_timestamp(0, 0).unwrap())),
            refresh_token_issued: Arc::new(RwLock::new(DateTime::from_timestamp(0, 0).unwrap())),
            access_token_timeout: 1800,  // 30 minutes
            refresh_token_timeout: 7 * 24 * 60 * 60,  // 7 days
        })
    }
    
    /// Initialize the token manager - load existing tokens if available
    pub async fn initialize(&self) -> Result<()> {
        if let Ok(saved_tokens) = self.load_tokens().await {
            let mut access_token = self.access_token.write().await;
            let mut refresh_token = self.refresh_token.write().await;
            let mut id_token = self.id_token.write().await;
            let mut access_token_issued = self.access_token_issued.write().await;
            let mut refresh_token_issued = self.refresh_token_issued.write().await;
            
            *access_token = Some(saved_tokens.token_dictionary.access_token);
            *refresh_token = Some(saved_tokens.token_dictionary.refresh_token);
            *id_token = Some(saved_tokens.token_dictionary.id_token);
            *access_token_issued = saved_tokens.access_token_issued;
            *refresh_token_issued = saved_tokens.refresh_token_issued;
            
            drop(access_token);
            drop(refresh_token);
            drop(id_token);
            drop(access_token_issued);
            drop(refresh_token_issued);
            
            // Check if tokens need updating
            self.update_tokens_if_needed().await?;
            
            let at_delta = self.access_token_timeout - 
                (Utc::now() - *self.access_token_issued.read().await).num_seconds();
            let rt_delta = self.refresh_token_timeout - 
                (Utc::now() - *self.refresh_token_issued.read().await).num_seconds();
            
            info!("Access token expires in {}H:{}M:{}S", 
                  at_delta / 3600, 
                  (at_delta % 3600) / 60, 
                  at_delta % 60);
            info!("Refresh token expires in {}H:{}M:{}S", 
                  rt_delta / 3600, 
                  (rt_delta % 3600) / 60, 
                  rt_delta % 60);
        } else {
            warn!("Token file does not exist or is invalid, need to authenticate");
            return Err(SchwabError::token("No valid tokens found, authentication required"));
        }
        
        Ok(())
    }
    
    /// Get the current access token
    pub async fn get_access_token(&self) -> Result<String> {
        let token = self.access_token.read().await;
        token.clone().ok_or_else(|| SchwabError::token("No access token available"))
    }
    
    /// Check if tokens need updating and update them if necessary
    pub async fn update_tokens_if_needed(&self) -> Result<bool> {
        let access_token_issued = *self.access_token_issued.read().await;
        let refresh_token_issued = *self.refresh_token_issued.read().await;
        
        let at_delta = self.access_token_timeout - 
            (Utc::now() - access_token_issued).num_seconds();
        let rt_delta = self.refresh_token_timeout - 
            (Utc::now() - refresh_token_issued).num_seconds();
        
        // Check if refresh token needs updating
        if rt_delta < 1800 {  // 30 minutes
            error!("Refresh token has expired or will expire soon!");
            return Err(SchwabError::token("Refresh token expired - manual authentication required"));
        }
        
        // Check if access token needs updating
        if at_delta < 61 {  // 1 minute
            info!("Access token has expired, updating automatically");
            self.update_access_token().await?;
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Generate the authorization URL for OAuth2 flow
    pub fn get_auth_url(&self) -> String {
        format!(
            "https://api.schwabapi.com/v1/oauth/authorize?client_id={}&redirect_uri={}",
            self.app_key, self.callback_url
        )
    }
    
    /// Exchange authorization code for tokens
    pub async fn exchange_code_for_tokens(&self, code: &str) -> Result<()> {
        let now = Utc::now();
        let response = self.post_oauth_token("authorization_code", code).await?;
        
        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            self.set_tokens(now, now, token_response).await?;
            info!("Successfully obtained new tokens");
            Ok(())
        } else {
            let error_text = response.text().await?;
            error!("Failed to exchange code for tokens: {}", error_text);
            Err(SchwabError::auth(format!("Token exchange failed: {}", error_text)))
        }
    }
    
    /// Update the access token using the refresh token
    pub async fn update_access_token(&self) -> Result<()> {
        let refresh_token = self.refresh_token.read().await;
        let refresh_token = refresh_token.as_ref()
            .ok_or_else(|| SchwabError::token("No refresh token available"))?;
        
        let response = self.post_oauth_token("refresh_token", refresh_token).await?;
        
        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            let access_token_issued = Utc::now();
            let refresh_token_issued = *self.refresh_token_issued.read().await;
            
            self.set_tokens(access_token_issued, refresh_token_issued, token_response).await?;
            info!("Access token updated successfully");
            Ok(())
        } else {
            let error_text = response.text().await?;
            error!("Failed to update access token: {}", error_text);
            Err(SchwabError::token(format!("Access token update failed: {}", error_text)))
        }
    }
    
    /// Start automatic token refresh background task
    pub async fn start_auto_refresh(&self) -> Result<()> {
        let manager = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(TokioDuration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = manager.update_tokens_if_needed().await {
                    error!("Token refresh failed: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Validate input parameters
    fn validate_input(app_key: &str, app_secret: &str, callback_url: &str) -> Result<()> {
        if app_key.is_empty() {
            return Err(SchwabError::config("app_key cannot be empty"));
        }
        if app_secret.is_empty() {
            return Err(SchwabError::config("app_secret cannot be empty"));
        }
        if callback_url.is_empty() {
            return Err(SchwabError::config("callback_url cannot be empty"));
        }
        
        if app_key.len() != 32 {
            return Err(SchwabError::config("app_key must be 32 characters long"));
        }
        if app_secret.len() != 16 {
            return Err(SchwabError::config("app_secret must be 16 characters long"));
        }
        if !callback_url.starts_with("https") {
            return Err(SchwabError::config("callback_url must use HTTPS"));
        }
        if callback_url.ends_with('/') {
            return Err(SchwabError::config("callback_url cannot end with '/'"));
        }
        
        Ok(())
    }
    
    /// Make OAuth token request
    async fn post_oauth_token(&self, grant_type: &str, code: &str) -> Result<reqwest::Response> {
        let auth_header = format!("{}:{}", self.app_key, self.app_secret);
        let auth_header = general_purpose::STANDARD.encode(auth_header);
        
        let mut form_data = HashMap::new();
        form_data.insert("grant_type", grant_type);
        
        if grant_type == "authorization_code" {
            form_data.insert("code", code);
            form_data.insert("redirect_uri", &self.callback_url);
        } else if grant_type == "refresh_token" {
            form_data.insert("refresh_token", code);
        }
        
        let response = self.http_client
            .post("https://api.schwabapi.com/v1/oauth/token")
            .header("Authorization", format!("Basic {}", auth_header))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form_data)
            .send()
            .await?;
        
        Ok(response)
    }
    
    /// Set tokens and save to file
    async fn set_tokens(
        &self,
        at_issued: DateTime<Utc>,
        rt_issued: DateTime<Utc>,
        token_response: TokenResponse,
    ) -> Result<()> {
        // Update in-memory tokens
        let mut access_token = self.access_token.write().await;
        let mut refresh_token = self.refresh_token.write().await;
        let mut id_token = self.id_token.write().await;
        let mut access_token_issued = self.access_token_issued.write().await;
        let mut refresh_token_issued = self.refresh_token_issued.write().await;
        
        *access_token = Some(token_response.access_token.clone());
        *refresh_token = Some(token_response.refresh_token.clone());
        *id_token = Some(token_response.id_token.clone());
        *access_token_issued = at_issued;
        *refresh_token_issued = rt_issued;
        
        // Save to file
        let saved_tokens = SavedTokens {
            access_token_issued: at_issued,
            refresh_token_issued: rt_issued,
            token_dictionary: token_response,
        };
        
        self.save_tokens(&saved_tokens).await?;
        
        Ok(())
    }
    
    /// Load tokens from file
    async fn load_tokens(&self) -> Result<SavedTokens> {
        let content = tokio::fs::read_to_string(&self.tokens_file).await?;
        let saved_tokens: SavedTokens = serde_json::from_str(&content)?;
        Ok(saved_tokens)
    }
    
    /// Save tokens to file
    async fn save_tokens(&self, tokens: &SavedTokens) -> Result<()> {
        let content = serde_json::to_string_pretty(tokens)?;
        tokio::fs::write(&self.tokens_file, content).await?;
        Ok(())
    }
}

impl Clone for TokenManager {
    fn clone(&self) -> Self {
        TokenManager {
            app_key: self.app_key.clone(),
            app_secret: self.app_secret.clone(),
            callback_url: self.callback_url.clone(),
            tokens_file: self.tokens_file.clone(),
            http_client: self.http_client.clone(),
            access_token: Arc::clone(&self.access_token),
            refresh_token: Arc::clone(&self.refresh_token),
            id_token: Arc::clone(&self.id_token),
            access_token_issued: Arc::clone(&self.access_token_issued),
            refresh_token_issued: Arc::clone(&self.refresh_token_issued),
            access_token_timeout: self.access_token_timeout,
            refresh_token_timeout: self.refresh_token_timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_token_manager_creation() {
        let manager = TokenManager::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com".to_string(),
            None,
        );
        
        assert!(manager.is_ok());
    }
    
    #[tokio::test]
    async fn test_invalid_app_key() {
        let manager = TokenManager::new(
            "short".to_string(),
            "b".repeat(16),
            "https://example.com".to_string(),
            None,
        );
        
        assert!(manager.is_err());
    }
    
    #[tokio::test]
    async fn test_invalid_callback_url() {
        let manager = TokenManager::new(
            "a".repeat(32),
            "b".repeat(16),
            "http://example.com".to_string(),  // HTTP instead of HTTPS
            None,
        );
        
        assert!(manager.is_err());
    }
    
    #[tokio::test]
    async fn test_auth_url_generation() {
        let manager = TokenManager::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com".to_string(),
            None,
        ).unwrap();
        
        let auth_url = manager.get_auth_url();
        assert!(auth_url.contains("client_id="));
        assert!(auth_url.contains("redirect_uri="));
        assert!(auth_url.contains("https://api.schwabapi.com/v1/oauth/authorize"));
    }
}