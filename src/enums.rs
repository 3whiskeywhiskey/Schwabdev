//! Enumerations for the Schwab API client

use serde::{Deserialize, Serialize};

/// Time formats supported by the API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFormat {
    /// ISO 8601 format (YYYY-MM-DDTHH:MM:SS.sssZ)
    #[serde(rename = "8601")]
    Iso8601,
    /// Unix timestamp (seconds since epoch)
    #[serde(rename = "epoch")]
    Epoch,
    /// Unix timestamp in milliseconds
    #[serde(rename = "epoch_ms")]
    EpochMs,
    /// Date format (YYYY-MM-DD)
    #[serde(rename = "YYYY-MM-DD")]
    YearMonthDay,
}

/// Order status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "AWAITING_PARENT_ORDER")]
    AwaitingParentOrder,
    #[serde(rename = "AWAITING_CONDITION")]
    AwaitingCondition,
    #[serde(rename = "AWAITING_MANUAL_REVIEW")]
    AwaitingManualReview,
    #[serde(rename = "ACCEPTED")]
    Accepted,
    #[serde(rename = "AWAITING_UR_OUT")]
    AwaitingUrOut,
    #[serde(rename = "PENDING_ACTIVATION")]
    PendingActivation,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "WORKING")]
    Working,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "PENDING_CANCEL")]
    PendingCancel,
    #[serde(rename = "CANCELED")]
    Canceled,
    #[serde(rename = "PENDING_REPLACE")]
    PendingReplace,
    #[serde(rename = "REPLACED")]
    Replaced,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "EXPIRED")]
    Expired,
}

/// Order type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
    #[serde(rename = "TRAILING_STOP")]
    TrailingStop,
    #[serde(rename = "MARKET_ON_CLOSE")]
    MarketOnClose,
    #[serde(rename = "EXERCISE")]
    Exercise,
    #[serde(rename = "TRAILING_STOP_LIMIT")]
    TrailingStopLimit,
    #[serde(rename = "NET_DEBIT")]
    NetDebit,
    #[serde(rename = "NET_CREDIT")]
    NetCredit,
    #[serde(rename = "NET_ZERO")]
    NetZero,
}

/// Duration for orders
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Duration {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "GOOD_TILL_CANCEL")]
    GoodTillCancel,
    #[serde(rename = "FILL_OR_KILL")]
    FillOrKill,
    #[serde(rename = "IMMEDIATE_OR_CANCEL")]
    ImmediateOrCancel,
    #[serde(rename = "END_OF_WEEK")]
    EndOfWeek,
    #[serde(rename = "END_OF_MONTH")]
    EndOfMonth,
    #[serde(rename = "NEXT_END_OF_MONTH")]
    NextEndOfMonth,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

/// Instruction for orders (buy/sell)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Instruction {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BUY_TO_COVER")]
    BuyToCover,
    #[serde(rename = "SELL_SHORT")]
    SellShort,
    #[serde(rename = "BUY_TO_OPEN")]
    BuyToOpen,
    #[serde(rename = "BUY_TO_CLOSE")]
    BuyToClose,
    #[serde(rename = "SELL_TO_OPEN")]
    SellToOpen,
    #[serde(rename = "SELL_TO_CLOSE")]
    SellToClose,
    #[serde(rename = "EXCHANGE")]
    Exchange,
}

/// Asset type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "CASH_EQUIVALENT")]
    CashEquivalent,
    #[serde(rename = "FIXED_INCOME")]
    FixedIncome,
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTURE_OPTION")]
    FutureOption,
    #[serde(rename = "FOREX")]
    Forex,
}

/// Market session enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Session {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SEAMLESS")]
    Seamless,
}

/// Option contract type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionType {
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "ALL")]
    All,
}

/// Option contract strategy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionStrategy {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    #[serde(rename = "COVERED")]
    Covered,
    #[serde(rename = "VERTICAL")]
    Vertical,
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "STRANGLE")]
    Strangle,
    #[serde(rename = "STRADDLE")]
    Straddle,
    #[serde(rename = "BUTTERFLY")]
    Butterfly,
    #[serde(rename = "CONDOR")]
    Condor,
    #[serde(rename = "DIAGONAL")]
    Diagonal,
    #[serde(rename = "COLLAR")]
    Collar,
    #[serde(rename = "ROLL")]
    Roll,
}

/// Option range
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptionRange {
    #[serde(rename = "ITM")]
    InTheMoney,
    #[serde(rename = "NTM")]
    NearTheMoney,
    #[serde(rename = "OTM")]
    OutOfTheMoney,
    #[serde(rename = "SAK")]
    StrikesAboveMarket,
    #[serde(rename = "SBK")]
    StrikesBelowMarket,
    #[serde(rename = "SNK")]
    StrikesNearMarket,
    #[serde(rename = "ALL")]
    All,
}

/// Frequency type for price history
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FrequencyType {
    #[serde(rename = "minute")]
    Minute,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}

/// Period type for price history
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeriodType {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "month")]
    Month,
    #[serde(rename = "year")]
    Year,
    #[serde(rename = "ytd")]
    YearToDate,
}

/// Movers sort criteria
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoversSort {
    #[serde(rename = "VOLUME")]
    Volume,
    #[serde(rename = "TRADES")]
    Trades,
    #[serde(rename = "PERCENT_CHANGE_UP")]
    PercentChangeUp,
    #[serde(rename = "PERCENT_CHANGE_DOWN")]
    PercentChangeDown,
}

/// Stream command types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamCommand {
    #[serde(rename = "SUBS")]
    Subscribe,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "UNSUBS")]
    Unsubscribe,
    #[serde(rename = "VIEW")]
    View,
    #[serde(rename = "LOGIN")]
    Login,
    #[serde(rename = "LOGOUT")]
    Logout,
}

/// Stream service types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamService {
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "LEVELONE_EQUITIES")]
    LevelOneEquities,
    #[serde(rename = "LEVELONE_OPTIONS")]
    LevelOneOptions,
    #[serde(rename = "LEVELONE_FUTURES")]
    LevelOneFutures,
    #[serde(rename = "LEVELONE_FUTURES_OPTIONS")]
    LevelOneFuturesOptions,
    #[serde(rename = "LEVELONE_FOREX")]
    LevelOneForex,
    #[serde(rename = "NYSE_BOOK")]
    NyseBook,
    #[serde(rename = "NASDAQ_BOOK")]
    NasdaqBook,
    #[serde(rename = "OPTIONS_BOOK")]
    OptionsBook,
    #[serde(rename = "CHART_EQUITY")]
    ChartEquity,
    #[serde(rename = "CHART_FUTURES")]
    ChartFutures,
    #[serde(rename = "SCREENER_EQUITY")]
    ScreenerEquity,
    #[serde(rename = "SCREENER_OPTIONS")]
    ScreenerOptions,
    #[serde(rename = "ACCOUNT_ACTIVITY")]
    AccountActivity,
}