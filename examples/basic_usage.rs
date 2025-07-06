//! Basic usage example for the Schwab API Rust client
//!
//! This example demonstrates:
//! - Authentication flow
//! - Getting account information
//! - Market data retrieval
//! - Placing orders
//! - Streaming real-time data

use chrono::Utc;
use schwab_api::*;
use schwab_api::enums::Duration;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Get credentials from environment variables
    let app_key = env::var("SCHWAB_APP_KEY")
        .expect("SCHWAB_APP_KEY environment variable required");
    let app_secret = env::var("SCHWAB_APP_SECRET")
        .expect("SCHWAB_APP_SECRET environment variable required");
    let callback_url = env::var("SCHWAB_CALLBACK_URL")
        .unwrap_or_else(|_| "https://127.0.0.1:8080".to_string());

    println!("🚀 Creating Schwab API client...");

    // Create the client
    let client = Client::new(app_key, app_secret, callback_url).await?;

    // Try to initialize with existing tokens
    match client.initialize().await {
        Ok(_) => {
            println!("✅ Successfully authenticated with existing tokens");
        }
        Err(_) => {
            println!("❌ No valid tokens found, manual authentication required");
            println!("🔗 Open this URL to authenticate: {}", client.get_auth_url());
            println!("📋 After authorizing, paste the callback URL here:");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let callback_url = input.trim();
            
            // Extract code from callback URL
            let code = extract_code_from_url(callback_url)?;
            client.exchange_code_for_tokens(&code).await?;
            
            println!("✅ Successfully authenticated!");
        }
    }

    // Example 1: Get account information
    println!("\n📊 Getting account information...");
    match get_account_info(&client).await {
        Ok(_) => println!("✅ Account information retrieved successfully"),
        Err(e) => println!("❌ Failed to get account info: {}", e),
    }

    // Example 2: Get market data
    println!("\n📈 Getting market data...");
    match get_market_data(&client).await {
        Ok(_) => println!("✅ Market data retrieved successfully"),
        Err(e) => println!("❌ Failed to get market data: {}", e),
    }

    // Example 3: Place a mock order (commented out for safety)
    // println!("\n💰 Placing a test order...");
    // match place_test_order(&client).await {
    //     Ok(_) => println!("✅ Order placed successfully"),
    //     Err(e) => println!("❌ Failed to place order: {}", e),
    // }

    // Example 4: Start streaming (commented out for brevity)
    // println!("\n📡 Starting real-time data stream...");
    // match start_streaming(&client).await {
    //     Ok(_) => println!("✅ Streaming started successfully"),
    //     Err(e) => println!("❌ Failed to start streaming: {}", e),
    // }

    println!("\n🎉 All examples completed!");
    Ok(())
}

/// Get account information
async fn get_account_info(client: &Client) -> Result<()> {
    // Get linked account numbers
    let account_numbers = client.get_account_numbers().await?;
    println!("🏦 Found {} linked accounts", account_numbers.len());

    // Get detailed account information
    let accounts = client.get_accounts(Some("positions")).await?;
    for account in &accounts {
        println!("💼 Account: {}", account.account_number);
        if let Some(balances) = &account.current_balances {
            if let Some(total_cash) = balances.total_cash {
                println!("💵 Total Cash: ${:.2}", total_cash);
            }
            if let Some(buying_power) = balances.buying_power {
                println!("💪 Buying Power: ${:.2}", buying_power);
            }
        }
        
        if let Some(positions) = &account.positions {
            println!("📋 Positions: {}", positions.len());
            for position in positions {
                println!("  📈 {}: {:.2} shares @ ${:.2}", 
                         position.instrument.symbol,
                         position.long_quantity,
                         position.average_price);
            }
        }
    }

    // Get user preferences (including streamer info)
    let preferences = client.get_user_preferences().await?;
    println!("⚙️ User preferences retrieved");
    if let Some(streamer_info) = &preferences.streamer_info {
        println!("📡 Streamer info available: {} endpoints", streamer_info.len());
    }

    Ok(())
}

/// Get market data
async fn get_market_data(client: &Client) -> Result<()> {
    // Get quotes for popular stocks
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string(), "GOOGL".to_string()];
    let quotes = client.get_quotes(&symbols, Some("quote"), None).await?;
    
    println!("📊 Stock Quotes:");
    for (symbol, quote) in &quotes {
        println!("  💹 {}: ${:.2} ({}{})", 
                 symbol,
                 quote.last_price.unwrap_or(0.0),
                 if quote.net_change.unwrap_or(0.0) >= 0.0 { "+" } else { "" },
                 quote.net_change.unwrap_or(0.0));
    }

    // Get price history for AAPL
    let end_date = Utc::now();
    let start_date = end_date - chrono::Duration::days(30);
    
    let price_history = client.get_price_history(
        "AAPL",
        Some(PeriodType::Month),
        Some(1),
        Some(FrequencyType::Daily),
        Some(1),
        Some(start_date),
        Some(end_date),
        None,
        None,
    ).await?;
    
    println!("📈 AAPL Price History: {} candles", price_history.candles.len());
    if let Some(latest_candle) = price_history.candles.last() {
        println!("  🕯️ Latest: O:${:.2} H:${:.2} L:${:.2} C:${:.2} V:{}", 
                 latest_candle.open,
                 latest_candle.high,
                 latest_candle.low,
                 latest_candle.close,
                 latest_candle.volume);
    }

    // Get option chain for AAPL
    let option_chain = client.get_option_chain(
        "AAPL",
        Some(OptionType::Call),
        Some(10),
        Some(true),
        Some(OptionStrategy::Single),
        None,
        None,
        Some(OptionRange::NearTheMoney),
        None,
        None,
    ).await?;
    
    println!("🔗 AAPL Option Chain: {} contracts", 
             option_chain.number_of_contracts.unwrap_or(0));

    Ok(())
}

/// Place a test order (for demonstration only - be careful with real orders!)
#[allow(dead_code)]
async fn place_test_order(client: &Client) -> Result<()> {
    // Get the first account
    let accounts = client.get_account_numbers().await?;
    if accounts.is_empty() {
        return Err(SchwabError::generic("No accounts found").into());
    }
    
    let account_hash = accounts[0].get("hashValue")
        .ok_or_else(|| SchwabError::generic("Account hash not found"))?;

    // Create a test order (this is just for demonstration)
    let order = Order {
        session: Session::Normal,
        duration: Duration::Day,
        order_type: OrderType::Limit,
        cancel_time: None,
        complex_order_strategy_type: None,
        quantity: 1.0,
        filled_quantity: 0.0,
        remaining_quantity: 1.0,
        requested_destination: None,
        destination_link_name: None,
        release_time: None,
        stop_price: None,
        stop_price_link_basis: None,
        stop_price_link_type: None,
        stop_price_offset: None,
        stop_type: None,
        price_link_basis: None,
        price_link_type: None,
        price: Some(150.0), // Limit price
        tax_lot_method: None,
        order_leg_collection: vec![OrderLeg {
            order_leg_type: None,
            leg_id: None,
            instrument: Instrument {
                asset_type: AssetType::Equity,
                cusip: None,
                symbol: "AAPL".to_string(),
                description: Some("Apple Inc.".to_string()),
                exchange: None,
                option_type: None,
                put_call: None,
                underlying_symbol: None,
                option_multiplier: None,
                option_root: None,
                option_deliverables: None,
                strike_price: None,
                expiration_date: None,
                expiration_type: None,
                exercise_type: None,
                bond_maturity_date: None,
                bond_interest_rate: None,
            },
            instruction: Instruction::Buy,
            position_effect: None,
            quantity: 1.0,
            quantity_type: None,
            dividend_date: None,
            to_symbol: None,
        }],
        activation_price: None,
        special_instruction: None,
        order_strategy_type: None,
        order_id: None,
        cancelable: None,
        editable: None,
        status: None,
        entered_time: None,
        close_time: None,
        account_number: None,
        order_activity_collection: None,
        replacing_order_collection: None,
        child_order_strategies: None,
        status_description: None,
    };

    // Note: This would place a real order! Be very careful!
    // let order_id = client.place_order(account_hash, &order).await?;
    // println!("📝 Order placed with ID: {}", order_id);

    println!("⚠️ Order placement skipped for safety (this is just a demo)");
    Ok(())
}

/// Start streaming real-time data
#[allow(dead_code)]
async fn start_streaming(client: &Client) -> Result<()> {
    // Define a message handler
    let message_handler = |message: String| {
        println!("📡 Received: {}", message);
    };

    // Start the stream
    client.stream.start(message_handler).await?;

    // Subscribe to level one equity data
    let symbols = vec!["AAPL".to_string(), "MSFT".to_string()];
    let fields = vec!["0".to_string(), "1".to_string(), "2".to_string(), "3".to_string()];
    
    client.stream.level_one_equities(&symbols, &fields, StreamCommand::Subscribe).await?;

    // Let it run for a bit
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Stop the stream
    client.stream.stop(true).await?;

    Ok(())
}

/// Extract authorization code from callback URL
fn extract_code_from_url(url: &str) -> Result<String> {
    let url = url::Url::parse(url)
        .map_err(|_| SchwabError::generic("Invalid callback URL"))?;
    
    let code = url.query_pairs()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.to_string())
        .ok_or_else(|| SchwabError::generic("Authorization code not found in URL"))?;
    
    // The code might be URL encoded, so we need to decode it
    let decoded_code = urlencoding::decode(&code)
        .map_err(|_| SchwabError::generic("Failed to decode authorization code"))?;
    
    Ok(format!("{}@", decoded_code)) // Schwab requires @ at the end
}