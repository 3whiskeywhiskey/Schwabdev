//! WebSocket streaming client for real-time data

use crate::enums::*;
use crate::error::{Result, SchwabError};
use crate::tokens::TokenManager;
use crate::types::*;
use futures_util::{SinkExt, StreamExt};
use log::{error, info, warn};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use url::Url;

/// WebSocket stream client for real-time data
pub struct StreamClient {
    token_manager: TokenManager,
    websocket: Arc<RwLock<Option<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>>>,
    streamer_info: Arc<RwLock<Option<StreamerInfo>>>,
    request_id: Arc<RwLock<u64>>,
    active: Arc<RwLock<bool>>,
    subscriptions: Arc<RwLock<HashMap<String, HashMap<String, Vec<String>>>>>,
    backoff_time: Arc<RwLock<f64>>,
    message_sender: Arc<RwLock<Option<mpsc::UnboundedSender<String>>>>,
}

impl StreamClient {
    /// Create a new stream client
    pub fn new(token_manager: TokenManager) -> Self {
        StreamClient {
            token_manager,
            websocket: Arc::new(RwLock::new(None)),
            streamer_info: Arc::new(RwLock::new(None)),
            request_id: Arc::new(RwLock::new(0)),
            active: Arc::new(RwLock::new(false)),
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            backoff_time: Arc::new(RwLock::new(2.0)),
            message_sender: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Start the streaming connection
    pub async fn start<F>(&self, receiver: F) -> Result<()>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let receiver = Arc::new(receiver);
        
        // Get streamer info
        let streamer_info = self.get_streamer_info().await?;
        let mut streamer_info_lock = self.streamer_info.write().await;
        *streamer_info_lock = Some(streamer_info.clone());
        drop(streamer_info_lock);
        
        // Create message channel
        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut message_sender = self.message_sender.write().await;
        *message_sender = Some(tx);
        drop(message_sender);
        
        // Start connection loop
        let stream_client = self.clone();
        let receiver_clone = Arc::clone(&receiver);
        
        tokio::spawn(async move {
            stream_client.connection_loop(receiver_clone, &mut rx).await;
        });
        
        Ok(())
    }
    
    /// Stop the streaming connection
    pub async fn stop(&self, clear_subscriptions: bool) -> Result<()> {
        if clear_subscriptions {
            let mut subscriptions = self.subscriptions.write().await;
            subscriptions.clear();
        }
        
        // Send logout message
        let logout_request = self.create_basic_request(
            StreamService::Admin,
            StreamCommand::Logout,
            None,
        ).await?;
        
        self.send_message(&logout_request).await?;
        
        let mut active = self.active.write().await;
        *active = false;
        
        Ok(())
    }
    
    /// Check if stream is active
    pub async fn is_active(&self) -> bool {
        *self.active.read().await
    }
    
    /// Send a subscription request
    pub async fn send_request(&self, request: &StreamRequest) -> Result<()> {
        // Record the request for reconnection
        self.record_request(request).await?;
        
        // If stream is active, send immediately
        if *self.active.read().await {
            self.send_message(request).await?;
        } else {
            info!("Stream is not active, request queued");
        }
        
        Ok(())
    }
    
    /// Send multiple requests
    pub async fn send_requests(&self, requests: &[StreamRequest]) -> Result<()> {
        for request in requests {
            self.send_request(request).await?;
        }
        Ok(())
    }
    
    // =================
    // Subscription Methods
    // =================
    
    /// Subscribe to level one equities
    pub async fn level_one_equities(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::LevelOneEquities,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to level one options
    pub async fn level_one_options(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::LevelOneOptions,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to level one futures
    pub async fn level_one_futures(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::LevelOneFutures,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to level one forex
    pub async fn level_one_forex(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::LevelOneForex,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to NYSE book
    pub async fn nyse_book(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::NyseBook,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to NASDAQ book
    pub async fn nasdaq_book(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::NasdaqBook,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to chart equity
    pub async fn chart_equity(
        &self,
        keys: &[String],
        fields: &[String],
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let request = self.create_basic_request(
            StreamService::ChartEquity,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    /// Subscribe to account activity
    pub async fn account_activity(
        &self,
        keys: Option<&[String]>,
        fields: Option<&[String]>,
        command: StreamCommand,
    ) -> Result<StreamRequest> {
        let default_keys = vec!["Account Activity".to_string()];
        let keys = keys.unwrap_or(&default_keys);
        let default_fields = vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()];
        let fields = fields.unwrap_or(&default_fields);
        
        let request = self.create_basic_request(
            StreamService::AccountActivity,
            command,
            Some(HashMap::from([
                ("keys".to_string(), keys.join(",")),
                ("fields".to_string(), fields.join(",")),
            ])),
        ).await?;
        
        self.send_request(&request).await?;
        Ok(request)
    }
    
    // =================
    // Internal Methods
    // =================
    
    /// Main connection loop
    async fn connection_loop<F>(
        &self,
        receiver: Arc<F>,
        message_rx: &mut mpsc::UnboundedReceiver<String>,
    ) where
        F: Fn(String) + Send + Sync + 'static,
    {
        loop {
            let start_time = std::time::Instant::now();
            
            match self.connect_and_run(Arc::clone(&receiver), message_rx).await {
                Ok(_) => {
                    info!("Stream connection closed normally");
                    break;
                }
                Err(e) => {
                    error!("Stream connection error: {}", e);
                    
                    // If connection failed quickly, it might be a configuration issue
                    if start_time.elapsed().as_secs() < 90 {
                        warn!("Stream crashed within 90 seconds, likely configuration issue");
                        break;
                    }
                    
                    // Exponential backoff
                    let backoff = *self.backoff_time.read().await;
                    warn!("Reconnecting in {} seconds...", backoff);
                    sleep(Duration::from_secs_f64(backoff)).await;
                    
                    let mut backoff_time = self.backoff_time.write().await;
                    *backoff_time = (*backoff_time * 2.0).min(128.0);
                }
            }
        }
    }
    
    /// Connect to WebSocket and run message loop
    async fn connect_and_run<F>(
        &self,
        receiver: Arc<F>,
        message_rx: &mut mpsc::UnboundedReceiver<String>,
    ) -> Result<()>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let streamer_info = self.streamer_info.read().await;
        let streamer_info = streamer_info.as_ref()
            .ok_or_else(|| SchwabError::stream("No streamer info available"))?;
        
        let url = Url::parse(&streamer_info.streamer_socket_url)?;
        info!("Connecting to streaming server: {}", url);
        
        let (ws_stream, _) = connect_async(url).await?;
        info!("Connected to streaming server");
        
        let (mut write, mut read) = ws_stream.split();
        
        // Store websocket reference
        let websocket = self.websocket.write().await;
        drop(websocket); // We can't store split websocket, so we'll handle it differently
        
        // Send login message
        let login_request = self.create_basic_request(
            StreamService::Admin,
            StreamCommand::Login,
            Some(HashMap::from([
                ("Authorization".to_string(), self.token_manager.get_access_token().await?),
                ("SchwabClientChannel".to_string(), streamer_info.schwab_client_channel.clone()),
                ("SchwabClientFunctionId".to_string(), streamer_info.schwab_client_function_id.clone()),
            ])),
        ).await?;
        
        let login_message = serde_json::to_string(&login_request)?;
        write.send(Message::Text(login_message)).await?;
        
        // Wait for login response
        if let Some(msg) = read.next().await {
            let msg = msg?;
            if let Message::Text(text) = msg {
                receiver(text);
            }
        }
        
        // Mark as active
        let mut active = self.active.write().await;
        *active = true;
        drop(active);
        
        // Send queued subscriptions
        self.send_queued_subscriptions(&mut write).await?;
        
        // Reset backoff time
        let mut backoff_time = self.backoff_time.write().await;
        *backoff_time = 2.0;
        drop(backoff_time);
        
        // Message handling loop
        loop {
            tokio::select! {
                // Handle incoming messages
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            receiver(text);
                        }
                        Some(Ok(Message::Close(_))) => {
                            info!("WebSocket connection closed by server");
                            break;
                        }
                        Some(Err(e)) => {
                            error!("WebSocket error: {}", e);
                            return Err(SchwabError::from(e));
                        }
                        None => {
                            warn!("WebSocket stream ended");
                            break;
                        }
                        _ => {} // Ignore other message types
                    }
                }
                
                // Handle outgoing messages
                outgoing = message_rx.recv() => {
                    if let Some(message) = outgoing {
                        write.send(Message::Text(message)).await?;
                    }
                }
            }
        }
        
        let mut active = self.active.write().await;
        *active = false;
        
        Ok(())
    }
    
    /// Send queued subscriptions
    async fn send_queued_subscriptions(
        &self,
        write: &mut futures_util::stream::SplitSink<
            WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
            Message,
        >,
    ) -> Result<()> {
        let subscriptions = self.subscriptions.read().await;
        
        for (service, subs) in subscriptions.iter() {
            // Group subscriptions by fields for efficiency
            let mut grouped: HashMap<String, Vec<String>> = HashMap::new();
            
            for (key, fields) in subs.iter() {
                let fields_str = fields.join(",");
                grouped.entry(fields_str).or_insert_with(Vec::new).push(key.clone());
            }
            
            // Send grouped requests
            for (fields, keys) in grouped {
                let request = StreamRequest {
                    service: service.clone(),
                    command: "ADD".to_string(),
                    requestid: self.next_request_id().await,
                    schwab_client_customer_id: self.get_customer_id().await?,
                    schwab_client_correl_id: self.get_correl_id().await?,
                    parameters: Some(HashMap::from([
                        ("keys".to_string(), keys.join(",")),
                        ("fields".to_string(), fields),
                    ])),
                };
                
                let message = serde_json::to_string(&json!({ "requests": [request] }))?;
                write.send(Message::Text(message)).await?;
                
                // Wait for response
                sleep(Duration::from_millis(100)).await;
            }
        }
        
        Ok(())
    }
    
    /// Get streamer info from API
    async fn get_streamer_info(&self) -> Result<StreamerInfo> {
        // This would typically make an API call to get streamer info
        // For now, we'll return a mock or require it to be provided
        Err(SchwabError::stream("Streamer info not available - call get_user_preferences() first"))
    }
    
    /// Create a basic stream request
    async fn create_basic_request(
        &self,
        service: StreamService,
        command: StreamCommand,
        parameters: Option<HashMap<String, String>>,
    ) -> Result<StreamRequest> {
        Ok(StreamRequest {
            service: serde_json::to_string(&service)?.trim_matches('"').to_string(),
            command: serde_json::to_string(&command)?.trim_matches('"').to_string(),
            requestid: self.next_request_id().await,
            schwab_client_customer_id: self.get_customer_id().await?,
            schwab_client_correl_id: self.get_correl_id().await?,
            parameters,
        })
    }
    
    /// Record a request for reconnection
    async fn record_request(&self, request: &StreamRequest) -> Result<()> {
        let mut subscriptions = self.subscriptions.write().await;
        let service = &request.service;
        let command = &request.command;
        
        if let Some(params) = &request.parameters {
            if let (Some(keys), Some(fields)) = (params.get("keys"), params.get("fields")) {
                let keys: Vec<String> = keys.split(',').map(|s| s.to_string()).collect();
                let fields: Vec<String> = fields.split(',').map(|s| s.to_string()).collect();
                
                let service_subs = subscriptions.entry(service.clone()).or_insert_with(HashMap::new);
                
                match command.as_str() {
                    "ADD" => {
                        for key in keys {
                            service_subs.insert(key, fields.clone());
                        }
                    }
                    "SUBS" => {
                        service_subs.clear();
                        for key in keys {
                            service_subs.insert(key, fields.clone());
                        }
                    }
                    "UNSUBS" => {
                        for key in keys {
                            service_subs.remove(&key);
                        }
                    }
                    _ => {} // Ignore other commands
                }
            }
        }
        
        Ok(())
    }
    
    /// Send a message through the WebSocket
    async fn send_message(&self, request: &StreamRequest) -> Result<()> {
        let message = serde_json::to_string(&json!({ "requests": [request] }))?;
        
        let message_sender = self.message_sender.read().await;
        if let Some(sender) = message_sender.as_ref() {
            sender.send(message).map_err(|e| SchwabError::stream(format!("Failed to send message: {}", e)))?;
        } else {
            return Err(SchwabError::stream("Message sender not available"));
        }
        
        Ok(())
    }
    
    /// Get next request ID
    async fn next_request_id(&self) -> u64 {
        let mut request_id = self.request_id.write().await;
        *request_id += 1;
        *request_id
    }
    
    /// Get customer ID from streamer info
    async fn get_customer_id(&self) -> Result<String> {
        let streamer_info = self.streamer_info.read().await;
        Ok(streamer_info.as_ref()
            .ok_or_else(|| SchwabError::stream("No streamer info available"))?
            .schwab_client_customer_id.clone())
    }
    
    /// Get correlation ID from streamer info
    async fn get_correl_id(&self) -> Result<String> {
        let streamer_info = self.streamer_info.read().await;
        Ok(streamer_info.as_ref()
            .ok_or_else(|| SchwabError::stream("No streamer info available"))?
            .schwab_client_correl_id.clone())
    }
    
    /// Set streamer info (called from main client)
    pub async fn set_streamer_info(&self, info: StreamerInfo) {
        let mut streamer_info = self.streamer_info.write().await;
        *streamer_info = Some(info);
    }
}

impl Clone for StreamClient {
    fn clone(&self) -> Self {
        StreamClient {
            token_manager: self.token_manager.clone(),
            websocket: Arc::clone(&self.websocket),
            streamer_info: Arc::clone(&self.streamer_info),
            request_id: Arc::clone(&self.request_id),
            active: Arc::clone(&self.active),
            subscriptions: Arc::clone(&self.subscriptions),
            backoff_time: Arc::clone(&self.backoff_time),
            message_sender: Arc::clone(&self.message_sender),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenManager;
    use tokio_test;
    
    #[tokio::test]
    async fn test_stream_client_creation() {
        let token_manager = TokenManager::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com".to_string(),
            None,
        ).unwrap();
        
        let stream_client = StreamClient::new(token_manager);
        assert!(!stream_client.is_active().await);
    }
    
    #[tokio::test]
    async fn test_request_id_generation() {
        let token_manager = TokenManager::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com".to_string(),
            None,
        ).unwrap();
        
        let stream_client = StreamClient::new(token_manager);
        
        let id1 = stream_client.next_request_id().await;
        let id2 = stream_client.next_request_id().await;
        
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }
}