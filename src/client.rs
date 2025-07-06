//! Main client for the Schwab API

use crate::enums::*;
use crate::error::{Result, SchwabError};
use crate::stream::StreamClient;
use crate::tokens::TokenManager;
use crate::types::*;
use chrono::{DateTime, Utc};

use reqwest::{Client as HttpClient, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;


/// Main Schwab API client
pub struct Client {
    token_manager: TokenManager,
    http_client: HttpClient,
    base_url: String,
    timeout: Duration,
    pub stream: StreamClient,
}

impl Client {
    /// Create a new Schwab API client
    pub async fn new(
        app_key: impl Into<String>,
        app_secret: impl Into<String>,
        callback_url: impl Into<String>,
    ) -> Result<Self> {
        let app_key = app_key.into();
        let app_secret = app_secret.into();
        let callback_url = callback_url.into();
        
        let token_manager = TokenManager::new(
            app_key,
            app_secret,
            callback_url,
            None,
        )?;
        
        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        
        let stream = StreamClient::new(token_manager.clone());
        
        let client = Client {
            token_manager,
            http_client,
            base_url: "https://api.schwabapi.com".to_string(),
            timeout: Duration::from_secs(10),
            stream,
        };
        
        Ok(client)
    }
    
    /// Initialize the client with existing tokens or require authentication
    pub async fn initialize(&self) -> Result<()> {
        self.token_manager.initialize().await?;
        self.token_manager.start_auto_refresh().await?;
        Ok(())
    }
    
    /// Get the authorization URL for OAuth2 flow
    pub fn get_auth_url(&self) -> String {
        self.token_manager.get_auth_url()
    }
    
    /// Exchange authorization code for tokens
    pub async fn exchange_code_for_tokens(&self, code: &str) -> Result<()> {
        self.token_manager.exchange_code_for_tokens(code).await
    }
    
    /// Set timeout for HTTP requests
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
    
    // =================
    // Account Methods
    // =================
    
    /// Get linked account numbers and hashes
    pub async fn get_account_numbers(&self) -> Result<Vec<HashMap<String, String>>> {
        let response = self.make_request("GET", "/trader/v1/accounts/accountNumbers", None, None::<()>).await?;
        let accounts: Vec<HashMap<String, String>> = response.json().await?;
        Ok(accounts)
    }
    
    /// Get details for all linked accounts
    pub async fn get_accounts(&self, fields: Option<&str>) -> Result<Vec<Account>> {
        let mut params = HashMap::new();
        if let Some(fields) = fields {
            params.insert("fields".to_string(), fields.to_string());
        }
        
        let response = self.make_request("GET", "/trader/v1/accounts", Some(params), None::<()>).await?;
        let accounts: Vec<Account> = response.json().await?;
        Ok(accounts)
    }
    
    /// Get details for a specific account
    pub async fn get_account(&self, account_hash: &str, fields: Option<&str>) -> Result<Account> {
        let mut params = HashMap::new();
        if let Some(fields) = fields {
            params.insert("fields".to_string(), fields.to_string());
        }
        
        let url = format!("/trader/v1/accounts/{}", account_hash);
        let response = self.make_request("GET", &url, Some(params), None::<()>).await?;
        let account: Account = response.json().await?;
        Ok(account)
    }
    
    /// Get orders for a specific account
    pub async fn get_account_orders(
        &self,
        account_hash: &str,
        from_entered_time: DateTime<Utc>,
        to_entered_time: DateTime<Utc>,
        max_results: Option<i32>,
        status: Option<OrderStatus>,
    ) -> Result<Vec<Order>> {
        let mut params = HashMap::new();
        params.insert("fromEnteredTime".to_string(), self.format_time(from_entered_time, TimeFormat::Iso8601));
        params.insert("toEnteredTime".to_string(), self.format_time(to_entered_time, TimeFormat::Iso8601));
        
        if let Some(max_results) = max_results {
            params.insert("maxResults".to_string(), max_results.to_string());
        }
        if let Some(status) = status {
            params.insert("status".to_string(), serde_json::to_string(&status)?);
        }
        
        let url = format!("/trader/v1/accounts/{}/orders", account_hash);
        let response = self.make_request("GET", &url, Some(params), None::<()>).await?;
        let orders: Vec<Order> = response.json().await?;
        Ok(orders)
    }
    
    /// Get all orders for all accounts
    pub async fn get_all_orders(
        &self,
        from_entered_time: DateTime<Utc>,
        to_entered_time: DateTime<Utc>,
        max_results: Option<i32>,
        status: Option<OrderStatus>,
    ) -> Result<Vec<Order>> {
        let mut params = HashMap::new();
        params.insert("fromEnteredTime".to_string(), self.format_time(from_entered_time, TimeFormat::Iso8601));
        params.insert("toEnteredTime".to_string(), self.format_time(to_entered_time, TimeFormat::Iso8601));
        
        if let Some(max_results) = max_results {
            params.insert("maxResults".to_string(), max_results.to_string());
        }
        if let Some(status) = status {
            params.insert("status".to_string(), serde_json::to_string(&status)?);
        }
        
        let response = self.make_request("GET", "/trader/v1/orders", Some(params), None::<()>).await?;
        let orders: Vec<Order> = response.json().await?;
        Ok(orders)
    }
    
    /// Place an order
    pub async fn place_order(&self, account_hash: &str, order: &Order) -> Result<String> {
        let url = format!("/trader/v1/accounts/{}/orders", account_hash);
        let response = self.make_request("POST", &url, None, Some(order)).await?;
        
        // Order ID is returned in the Location header
        if let Some(location) = response.headers().get("Location") {
            if let Ok(location_str) = location.to_str() {
                if let Some(order_id) = location_str.split('/').last() {
                    return Ok(order_id.to_string());
                }
            }
        }
        
        Err(SchwabError::generic("Order placed but order ID not found in response"))
    }
    
    /// Get order details
    pub async fn get_order(&self, account_hash: &str, order_id: &str) -> Result<Order> {
        let url = format!("/trader/v1/accounts/{}/orders/{}", account_hash, order_id);
        let response = self.make_request("GET", &url, None, None::<()>).await?;
        let order: Order = response.json().await?;
        Ok(order)
    }
    
    /// Cancel an order
    pub async fn cancel_order(&self, account_hash: &str, order_id: &str) -> Result<()> {
        let url = format!("/trader/v1/accounts/{}/orders/{}", account_hash, order_id);
        self.make_request("DELETE", &url, None, None::<()>).await?;
        Ok(())
    }
    
    /// Replace an order
    pub async fn replace_order(&self, account_hash: &str, order_id: &str, order: &Order) -> Result<()> {
        let url = format!("/trader/v1/accounts/{}/orders/{}", account_hash, order_id);
        self.make_request("PUT", &url, None, Some(order)).await?;
        Ok(())
    }
    
    /// Get transactions for an account
    pub async fn get_transactions(
        &self,
        account_hash: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        types: &str,
        symbol: Option<&str>,
    ) -> Result<Vec<Transaction>> {
        let mut params = HashMap::new();
        params.insert("startDate".to_string(), self.format_time(start_date, TimeFormat::Iso8601));
        params.insert("endDate".to_string(), self.format_time(end_date, TimeFormat::Iso8601));
        params.insert("types".to_string(), types.to_string());
        
        if let Some(symbol) = symbol {
            params.insert("symbol".to_string(), symbol.to_string());
        }
        
        let url = format!("/trader/v1/accounts/{}/transactions", account_hash);
        let response = self.make_request("GET", &url, Some(params), None::<()>).await?;
        let transactions: Vec<Transaction> = response.json().await?;
        Ok(transactions)
    }
    
    /// Get transaction details
    pub async fn get_transaction(&self, account_hash: &str, transaction_id: &str) -> Result<Transaction> {
        let url = format!("/trader/v1/accounts/{}/transactions/{}", account_hash, transaction_id);
        let response = self.make_request("GET", &url, None, None::<()>).await?;
        let transaction: Transaction = response.json().await?;
        Ok(transaction)
    }
    
    /// Get user preferences
    pub async fn get_user_preferences(&self) -> Result<UserPreferences> {
        let response = self.make_request("GET", "/trader/v1/userPreference", None, None::<()>).await?;
        let preferences: UserPreferences = response.json().await?;
        
        // Set streamer info for the stream client
        if let Some(streamer_info) = preferences.streamer_info.as_ref().and_then(|info| info.first()) {
            self.stream.set_streamer_info(streamer_info.clone()).await;
        }
        
        Ok(preferences)
    }
    
    // =================
    // Market Data Methods
    // =================
    
    /// Get quotes for multiple symbols
    pub async fn get_quotes(
        &self,
        symbols: &[String],
        fields: Option<&str>,
        indicative: Option<bool>,
    ) -> Result<HashMap<String, Quote>> {
        let mut params = HashMap::new();
        params.insert("symbols".to_string(), symbols.join(","));
        
        if let Some(fields) = fields {
            params.insert("fields".to_string(), fields.to_string());
        }
        if let Some(indicative) = indicative {
            params.insert("indicative".to_string(), indicative.to_string());
        }
        
        let response = self.make_request("GET", "/marketdata/v1/quotes", Some(params), None::<()>).await?;
        let quotes: HashMap<String, Quote> = response.json().await?;
        Ok(quotes)
    }
    
    /// Get quote for a single symbol
    pub async fn get_quote(&self, symbol: &str, fields: Option<&str>) -> Result<Quote> {
        let mut params = HashMap::new();
        if let Some(fields) = fields {
            params.insert("fields".to_string(), fields.to_string());
        }
        
        let symbol_encoded = urlencoding::encode(symbol);
        let url = format!("/marketdata/v1/{}/quotes", symbol_encoded);
        let response = self.make_request("GET", &url, Some(params), None::<()>).await?;
        let quote: Quote = response.json().await?;
        Ok(quote)
    }
    
    /// Get option chain
    pub async fn get_option_chain(
        &self,
        symbol: &str,
        contract_type: Option<OptionType>,
        strike_count: Option<i32>,
        include_underlying_quote: Option<bool>,
        strategy: Option<OptionStrategy>,
        interval: Option<f64>,
        strike: Option<f64>,
        range: Option<OptionRange>,
        from_date: Option<DateTime<Utc>>,
        to_date: Option<DateTime<Utc>>,
    ) -> Result<OptionChain> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(contract_type) = contract_type {
            params.insert("contractType".to_string(), serde_json::to_string(&contract_type)?);
        }
        if let Some(strike_count) = strike_count {
            params.insert("strikeCount".to_string(), strike_count.to_string());
        }
        if let Some(include_underlying_quote) = include_underlying_quote {
            params.insert("includeUnderlyingQuote".to_string(), include_underlying_quote.to_string());
        }
        if let Some(strategy) = strategy {
            params.insert("strategy".to_string(), serde_json::to_string(&strategy)?);
        }
        if let Some(interval) = interval {
            params.insert("interval".to_string(), interval.to_string());
        }
        if let Some(strike) = strike {
            params.insert("strike".to_string(), strike.to_string());
        }
        if let Some(range) = range {
            params.insert("range".to_string(), serde_json::to_string(&range)?);
        }
        if let Some(from_date) = from_date {
            params.insert("fromDate".to_string(), self.format_time(from_date, TimeFormat::YearMonthDay));
        }
        if let Some(to_date) = to_date {
            params.insert("toDate".to_string(), self.format_time(to_date, TimeFormat::YearMonthDay));
        }
        
        let response = self.make_request("GET", "/marketdata/v1/chains", Some(params), None::<()>).await?;
        let option_chain: OptionChain = response.json().await?;
        Ok(option_chain)
    }
    
    /// Get price history
    pub async fn get_price_history(
        &self,
        symbol: &str,
        period_type: Option<PeriodType>,
        period: Option<i32>,
        frequency_type: Option<FrequencyType>,
        frequency: Option<i32>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        need_extended_hours_data: Option<bool>,
        need_previous_close: Option<bool>,
    ) -> Result<PriceHistory> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(period_type) = period_type {
            params.insert("periodType".to_string(), serde_json::to_string(&period_type)?);
        }
        if let Some(period) = period {
            params.insert("period".to_string(), period.to_string());
        }
        if let Some(frequency_type) = frequency_type {
            params.insert("frequencyType".to_string(), serde_json::to_string(&frequency_type)?);
        }
        if let Some(frequency) = frequency {
            params.insert("frequency".to_string(), frequency.to_string());
        }
        if let Some(start_date) = start_date {
            params.insert("startDate".to_string(), self.format_time(start_date, TimeFormat::EpochMs));
        }
        if let Some(end_date) = end_date {
            params.insert("endDate".to_string(), self.format_time(end_date, TimeFormat::EpochMs));
        }
        if let Some(need_extended_hours_data) = need_extended_hours_data {
            params.insert("needExtendedHoursData".to_string(), need_extended_hours_data.to_string());
        }
        if let Some(need_previous_close) = need_previous_close {
            params.insert("needPreviousClose".to_string(), need_previous_close.to_string());
        }
        
        let response = self.make_request("GET", "/marketdata/v1/pricehistory", Some(params), None::<()>).await?;
        let price_history: PriceHistory = response.json().await?;
        Ok(price_history)
    }
    
    /// Get market movers
    pub async fn get_movers(
        &self,
        symbol: &str,
        sort: Option<MoversSort>,
        frequency: Option<i32>,
    ) -> Result<Vec<Mover>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        
        if let Some(sort) = sort {
            params.insert("sort".to_string(), serde_json::to_string(&sort)?);
        }
        if let Some(frequency) = frequency {
            params.insert("frequency".to_string(), frequency.to_string());
        }
        
        let response = self.make_request("GET", "/marketdata/v1/movers", Some(params), None::<()>).await?;
        let movers: Vec<Mover> = response.json().await?;
        Ok(movers)
    }
    
    /// Get market hours
    pub async fn get_market_hours(
        &self,
        symbols: &[String],
        date: Option<DateTime<Utc>>,
    ) -> Result<HashMap<String, Value>> {
        let mut params = HashMap::new();
        params.insert("symbols".to_string(), symbols.join(","));
        
        if let Some(date) = date {
            params.insert("date".to_string(), self.format_time(date, TimeFormat::YearMonthDay));
        }
        
        let response = self.make_request("GET", "/marketdata/v1/markets", Some(params), None::<()>).await?;
        let market_hours: HashMap<String, Value> = response.json().await?;
        Ok(market_hours)
    }
    
    /// Get market hours for a specific market
    pub async fn get_market_hour(
        &self,
        market_id: &str,
        date: Option<DateTime<Utc>>,
    ) -> Result<Value> {
        let mut params = HashMap::new();
        if let Some(date) = date {
            params.insert("date".to_string(), self.format_time(date, TimeFormat::YearMonthDay));
        }
        
        let url = format!("/marketdata/v1/markets/{}", market_id);
        let response = self.make_request("GET", &url, Some(params), None::<()>).await?;
        let market_hour: Value = response.json().await?;
        Ok(market_hour)
    }
    
    /// Search instruments
    pub async fn search_instruments(&self, symbol: &str, projection: &str) -> Result<HashMap<String, Value>> {
        let mut params = HashMap::new();
        params.insert("symbol".to_string(), symbol.to_string());
        params.insert("projection".to_string(), projection.to_string());
        
        let response = self.make_request("GET", "/marketdata/v1/instruments", Some(params), None::<()>).await?;
        let instruments: HashMap<String, Value> = response.json().await?;
        Ok(instruments)
    }
    
    /// Get instrument by CUSIP
    pub async fn get_instrument_by_cusip(&self, cusip: &str) -> Result<HashMap<String, Value>> {
        let url = format!("/marketdata/v1/instruments/{}", cusip);
        let response = self.make_request("GET", &url, None, None::<()>).await?;
        let instrument: HashMap<String, Value> = response.json().await?;
        Ok(instrument)
    }
    
    // =================
    // Helper Methods
    // =================
    
    /// Make an authenticated HTTP request
    async fn make_request(
        &self,
        method: &str,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
        body: Option<impl serde::Serialize>,
    ) -> Result<Response> {
        let access_token = self.token_manager.get_access_token().await?;
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = match method {
            "GET" => self.http_client.get(&url),
            "POST" => self.http_client.post(&url),
            "PUT" => self.http_client.put(&url),
            "DELETE" => self.http_client.delete(&url),
            _ => return Err(SchwabError::generic("Unsupported HTTP method")),
        };
        
        request = request
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/json")
            .timeout(self.timeout);
        
        if let Some(params) = params {
            request = request.query(&params);
        }
        
        if let Some(body) = body {
            request = request
                .header("Content-Type", "application/json")
                .json(&body);
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status().as_u16();
            let error_text = response.text().await?;
            Err(SchwabError::api(status, error_text))
        }
    }
    
    /// Format DateTime to string based on TimeFormat
    fn format_time(&self, dt: DateTime<Utc>, format: TimeFormat) -> String {
        match format {
            TimeFormat::Iso8601 => {
                // Format: YYYY-MM-DDTHH:MM:SS.sssZ
                format!("{}Z", dt.format("%Y-%m-%dT%H:%M:%S%.3f"))
            }
            TimeFormat::Epoch => dt.timestamp().to_string(),
            TimeFormat::EpochMs => (dt.timestamp_millis()).to_string(),
            TimeFormat::YearMonthDay => dt.format("%Y-%m-%d").to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use tokio_test;
    
    #[tokio::test]
    async fn test_client_creation() {
        let client = Client::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com",
        ).await;
        
        assert!(client.is_ok());
    }
    
    #[tokio::test]
    async fn test_auth_url() {
        let client = Client::new(
            "a".repeat(32),
            "b".repeat(16),
            "https://example.com",
        ).await.unwrap();
        
        let auth_url = client.get_auth_url();
        assert!(auth_url.contains("client_id="));
        assert!(auth_url.contains("redirect_uri="));
    }
    
    #[test]
    fn test_time_formatting() {
        let client = Client {
            token_manager: TokenManager::new(
                "a".repeat(32),
                "b".repeat(16),
                "https://example.com".to_string(),
                None,
            ).unwrap(),
            http_client: HttpClient::new(),
            base_url: "https://api.schwabapi.com".to_string(),
            timeout: Duration::from_secs(10),
            stream: StreamClient::new(TokenManager::new(
                "a".repeat(32),
                "b".repeat(16),
                "https://example.com".to_string(),
                None,
            ).unwrap()),
        };
        
        let dt = DateTime::from_timestamp(1609459200, 0).unwrap(); // 2021-01-01T00:00:00Z
        
        assert_eq!(client.format_time(dt, TimeFormat::Epoch), "1609459200");
        assert_eq!(client.format_time(dt, TimeFormat::EpochMs), "1609459200000");
        assert_eq!(client.format_time(dt, TimeFormat::YearMonthDay), "2021-01-01");
    }
}