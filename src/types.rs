//! Type definitions for the Schwab API client

use crate::enums::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_hash: String,
    pub account_number: String,
    pub account_type: String,
    pub current_balances: Option<AccountBalances>,
    pub initial_balances: Option<AccountBalances>,
    pub positions: Option<Vec<Position>>,
    pub orders: Option<Vec<Order>>,
}

/// Account balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalances {
    pub account_value: Option<f64>,
    pub available_cash: Option<f64>,
    pub available_cash_non_marginable: Option<f64>,
    pub buying_power: Option<f64>,
    pub buying_power_non_marginable: Option<f64>,
    pub cash_balance: Option<f64>,
    pub cash_available_for_trading: Option<f64>,
    pub cash_receipts: Option<f64>,
    pub long_option_market_value: Option<f64>,
    pub liquid_net_worth: Option<f64>,
    pub long_market_value: Option<f64>,
    pub money_market_fund: Option<f64>,
    pub mutual_fund_value: Option<f64>,
    pub short_option_market_value: Option<f64>,
    pub short_market_value: Option<f64>,
    pub total_cash: Option<f64>,
    pub is_in_call: Option<bool>,
    pub unsettled_cash: Option<f64>,
    pub pending_deposits: Option<f64>,
    pub margin_balance: Option<f64>,
    pub short_balance: Option<f64>,
    pub reg_t_call: Option<f64>,
}

/// Position information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub short_quantity: f64,
    pub average_price: f64,
    pub current_day_profit_loss: Option<f64>,
    pub current_day_profit_loss_percentage: Option<f64>,
    pub long_quantity: f64,
    pub settled_long_quantity: f64,
    pub settled_short_quantity: f64,
    pub agedQuantity: Option<f64>,
    pub instrument: Instrument,
    pub market_value: Option<f64>,
    pub main_enance_requirement: Option<f64>,
    pub previous_session_long_quantity: Option<f64>,
}

/// Instrument information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub asset_type: AssetType,
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub exchange: Option<String>,
    pub option_type: Option<OptionType>,
    pub put_call: Option<String>,
    pub underlying_symbol: Option<String>,
    pub option_multiplier: Option<f64>,
    pub option_root: Option<String>,
    pub option_deliverables: Option<Vec<OptionDeliverable>>,
    pub strike_price: Option<f64>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub expiration_type: Option<String>,
    pub exercise_type: Option<String>,
    pub bond_maturity_date: Option<DateTime<Utc>>,
    pub bond_interest_rate: Option<f64>,
}

/// Option deliverable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionDeliverable {
    pub symbol: String,
    pub deliverable_units: f64,
    pub currency_type: String,
    pub asset_type: AssetType,
}

/// Order information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub session: Session,
    pub duration: Duration,
    pub order_type: OrderType,
    pub cancel_time: Option<DateTime<Utc>>,
    pub complex_order_strategy_type: Option<String>,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub remaining_quantity: f64,
    pub requested_destination: Option<String>,
    pub destination_link_name: Option<String>,
    pub release_time: Option<DateTime<Utc>>,
    pub stop_price: Option<f64>,
    pub stop_price_link_basis: Option<String>,
    pub stop_price_link_type: Option<String>,
    pub stop_price_offset: Option<f64>,
    pub stop_type: Option<String>,
    pub price_link_basis: Option<String>,
    pub price_link_type: Option<String>,
    pub price: Option<f64>,
    pub tax_lot_method: Option<String>,
    pub order_leg_collection: Vec<OrderLeg>,
    pub activation_price: Option<f64>,
    pub special_instruction: Option<String>,
    pub order_strategy_type: Option<String>,
    pub order_id: Option<i64>,
    pub cancelable: Option<bool>,
    pub editable: Option<bool>,
    pub status: Option<OrderStatus>,
    pub entered_time: Option<DateTime<Utc>>,
    pub close_time: Option<DateTime<Utc>>,
    pub account_number: Option<String>,
    pub order_activity_collection: Option<Vec<OrderActivity>>,
    pub replacing_order_collection: Option<Vec<Order>>,
    pub child_order_strategies: Option<Vec<Order>>,
    pub status_description: Option<String>,
}

/// Order leg information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderLeg {
    pub order_leg_type: Option<String>,
    pub leg_id: Option<i64>,
    pub instrument: Instrument,
    pub instruction: Instruction,
    pub position_effect: Option<String>,
    pub quantity: f64,
    pub quantity_type: Option<String>,
    pub dividend_date: Option<DateTime<Utc>>,
    pub to_symbol: Option<String>,
}

/// Order activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderActivity {
    pub activity_type: Option<String>,
    pub activity_id: Option<i64>,
    pub execution_type: Option<String>,
    pub quantity: Option<f64>,
    pub order_remaining_quantity: Option<f64>,
    pub execution_legs: Option<Vec<ExecutionLeg>>,
}

/// Execution leg
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLeg {
    pub leg_id: Option<i64>,
    pub price: Option<f64>,
    pub quantity: Option<f64>,
    pub mismarked_quantity: Option<f64>,
    pub instrument: Option<Instrument>,
    pub time: Option<DateTime<Utc>>,
}

/// Quote information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub asset_type: AssetType,
    pub asset_main_type: Option<String>,
    pub cusip: Option<String>,
    pub symbol: String,
    pub description: Option<String>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<i64>,
    pub bid_id: Option<String>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<i64>,
    pub ask_id: Option<String>,
    pub last_price: Option<f64>,
    pub last_size: Option<i64>,
    pub last_id: Option<String>,
    pub open_price: Option<f64>,
    pub high_price: Option<f64>,
    pub low_price: Option<f64>,
    pub bid_tick: Option<String>,
    pub close_price: Option<f64>,
    pub net_change: Option<f64>,
    pub total_volume: Option<i64>,
    pub quote_time_in_long: Option<i64>,
    pub trade_time_in_long: Option<i64>,
    pub mark: Option<f64>,
    pub exchange: Option<String>,
    pub exchange_name: Option<String>,
    pub marginable: Option<bool>,
    pub shortable: Option<bool>,
    pub volatility: Option<f64>,
    pub digits: Option<i32>,
    pub n_a_v: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub div_amount: Option<f64>,
    pub div_yield: Option<f64>,
    pub div_date: Option<String>,
    pub security_status: Option<String>,
    pub regular_market_last_price: Option<f64>,
    pub regular_market_last_size: Option<i64>,
    pub regular_market_net_change: Option<f64>,
    pub regular_market_trade_time_in_long: Option<i64>,
    pub net_percent_change_in_double: Option<f64>,
    pub mark_change_in_double: Option<f64>,
    pub mark_percent_change_in_double: Option<f64>,
    pub regular_market_percent_change_in_double: Option<f64>,
    pub delayed: Option<bool>,
    pub realtime_entitled: Option<bool>,
}

/// Option chain response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Option<Underlying>,
    pub strategy: OptionStrategy,
    pub interval: Option<f64>,
    pub is_delayed: Option<bool>,
    pub is_index: Option<bool>,
    pub interest_rate: Option<f64>,
    pub underlying_price: Option<f64>,
    pub volatility: Option<f64>,
    pub days_to_expiration: Option<f64>,
    pub number_of_contracts: Option<i32>,
    pub call_exp_date_map: Option<HashMap<String, HashMap<String, Vec<OptionContract>>>>,
    pub put_exp_date_map: Option<HashMap<String, HashMap<String, Vec<OptionContract>>>>,
}

/// Underlying asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Underlying {
    pub ask: Option<f64>,
    pub ask_size: Option<i64>,
    pub bid: Option<f64>,
    pub bid_size: Option<i64>,
    pub change: Option<f64>,
    pub close: Option<f64>,
    pub delayed: Option<bool>,
    pub description: Option<String>,
    pub exchange_name: Option<String>,
    pub fifty_two_week_high: Option<f64>,
    pub fifty_two_week_low: Option<f64>,
    pub high_price: Option<f64>,
    pub last: Option<f64>,
    pub low_price: Option<f64>,
    pub mark: Option<f64>,
    pub mark_change: Option<f64>,
    pub mark_percent_change: Option<f64>,
    pub open_price: Option<f64>,
    pub percent_change: Option<f64>,
    pub quote_time: Option<i64>,
    pub symbol: String,
    pub total_volume: Option<i64>,
    pub trade_time: Option<i64>,
}

/// Option contract information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionContract {
    pub put_call: OptionType,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub bid_ask_size: Option<String>,
    pub last_size: i64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: i64,
    pub trade_date: Option<String>,
    pub trade_time_in_long: i64,
    pub quote_time_in_long: i64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub open_interest: i64,
    pub time_value: f64,
    pub expiration_date: i64,
    pub expiration_type: String,
    pub last_trading_day: i64,
    pub multiplier: f64,
    pub strike_price: f64,
    pub contract_type: String,
    pub underlying: String,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub in_the_money: bool,
    pub intrinsic_value: f64,
    pub penny_pilot: bool,
    pub non_standard: bool,
    pub mini: bool,
}

/// Price history response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceHistory {
    pub candles: Vec<Candle>,
    pub symbol: String,
    pub empty: bool,
    pub previous_close: Option<f64>,
    pub previous_close_date: Option<i64>,
}

/// Candle data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candle {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub datetime: i64,
}

/// Market mover information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub change: f64,
    pub description: String,
    pub direction: String,
    pub last: f64,
    pub symbol: String,
    pub total_volume: i64,
}

/// Transaction information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub activity_id: Option<i64>,
    pub time: Option<DateTime<Utc>>,
    pub user: Option<String>,
    pub description: Option<String>,
    pub account_number: Option<String>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub trade_date: Option<DateTime<Utc>>,
    pub settlement_date: Option<DateTime<Utc>>,
    pub position_id: Option<i64>,
    pub order_id: Option<i64>,
    pub net_amount: Option<f64>,
    pub activity_type: Option<String>,
    pub transfer_items: Option<Vec<TransferItem>>,
}

/// Transfer item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferItem {
    pub instrument: Option<Instrument>,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub cost: Option<f64>,
    pub parent_order_key: Option<String>,
    pub parent_child_indicator: Option<String>,
    pub instruction: Option<Instruction>,
    pub position_effect: Option<String>,
}

/// Stream message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamMessage {
    pub data: Option<Vec<StreamData>>,
    pub notify: Option<Vec<StreamNotify>>,
    pub snapshot: Option<Vec<StreamSnapshot>>,
}

/// Stream data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamData {
    pub service: String,
    pub timestamp: i64,
    pub command: String,
    pub content: Vec<HashMap<String, serde_json::Value>>,
}

/// Stream notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamNotify {
    pub service: String,
    pub timestamp: i64,
    pub command: String,
    pub content: HashMap<String, serde_json::Value>,
}

/// Stream snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamSnapshot {
    pub service: String,
    pub timestamp: i64,
    pub command: String,
    pub content: Vec<HashMap<String, serde_json::Value>>,
}

/// Stream request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamRequest {
    pub service: String,
    pub command: String,
    pub requestid: u64,
    pub schwab_client_customer_id: String,
    pub schwab_client_correl_id: String,
    pub parameters: Option<HashMap<String, String>>,
}

/// Stream requests wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRequests {
    pub requests: Vec<StreamRequest>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferences {
    pub accounts: Option<Vec<AccountPreferences>>,
    pub streamer_info: Option<Vec<StreamerInfo>>,
    pub offers: Option<Vec<Offer>>,
}

/// Account preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPreferences {
    pub account_number: String,
    pub primary_account: bool,
    pub r#type: String,
    pub nickname: Option<String>,
    pub account_color: Option<String>,
    pub display_account_number: bool,
    pub auto_position_effect: bool,
}

/// Streamer information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerInfo {
    pub streamer_socket_url: String,
    pub schwab_client_customer_id: String,
    pub schwab_client_correl_id: String,
    pub schwab_client_channel: String,
    pub schwab_client_function_id: String,
}

/// Offer information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    pub level2_permissions: bool,
    pub mkt_data_permission: String,
}

/// Token response from OAuth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub scope: String,
    pub refresh_token_expires_in: i64,
}

/// Saved token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedTokens {
    pub access_token_issued: DateTime<Utc>,
    pub refresh_token_issued: DateTime<Utc>,
    pub token_dictionary: TokenResponse,
}