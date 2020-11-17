use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt;

use serde::de::{self, Visitor};

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenRequest {
    pub grant_type: String,
    pub refresh_token: String,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenResponse {
    pub access_token: String,
    //refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: String,
    //refresh_token_expires_in: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteStock {
    //Equity:
    pub symbol: String,
    pub description: String,
    pub bidPrice: f64,
    pub bidSize: f64,
    pub bidId: String,
    pub askPrice: f64,
    pub askSize: f64,
    pub askId: String,
    pub lastPrice: f64,
    pub lastSize: f64,
    pub lastId: String,
    pub openPrice: f64,
    pub highPrice: f64,
    pub lowPrice: f64,
    pub closePrice: f64,
    pub netChange: f64,
    pub totalVolume: f64,
    pub quoteTimeInLong: i64,
    pub tradeTimeInLong: i64,
    pub mark: f64,
    pub exchange: String,
    pub exchangeName: String,
    pub marginable: bool,
    pub shortable: bool,
    pub volatility: f64,
    pub digits: f64,
    #[serde(rename(deserialize = "52WkHigh"))]
    pub YearHigh: f64,
    #[serde(rename(deserialize = "52WkLow"))]
    pub YearLow: f64,
    pub peRatio: f64,
    pub divAmount: f64,
    pub divYield: f64,
    pub divDate: String,
    pub securityStatus: String,
    pub regularMarketLastPrice: f64,
    pub regularMarketLastSize: f64,
    pub regularMarketNetChange: f64,
    pub regularMarketTradeTimeInLong: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteOption {
    pub symbol: String,
    pub description: String,
    pub bidPrice: f64,
    pub bidSize: f64,
    pub askPrice: f64,
    pub askSize: f64,
    pub lastPrice: f64,
    pub lastSize: f64,
    pub openPrice: f64,
    pub highPrice: f64,
    pub lowPrice: f64,
    pub closePrice: f64,
    pub netChange: f64,
    pub totalVolume: f64,
    pub quoteTimeInLong: i64,
    pub tradeTimeInLong: i64,
    pub mark: f64,
    pub openInterest: f64,
    pub volatility: f64,
    pub moneyIntrinsicValue: f64,
    pub multiplier: f64,
    pub strikePrice: f64,
    pub contractType: String,
    pub underlying: String,
    pub timeValue: f64,
    pub deliverables: String,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub securityStatus: String,
    pub theoreticalOptionValue: f64,
    pub underlyingPrice: f64,
    pub uvExpirationType: String,
    pub exchange: String,
    pub exchangeName: String,
    pub settlementType: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceHistory {
    pub candles: Vec<Candle>,
    pub empty: bool,
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candle {
    pub datetime: i64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: f64,
}

impl PartialEq for Candle {
    fn eq(&self, other: &Self) -> bool {
        const TOLERANCE: f64 = 0.001;
        self.datetime == other.datetime
            && (self.close - other.close).abs() < TOLERANCE
            && (self.high - other.high).abs() < TOLERANCE
            && (self.low - other.low).abs() < TOLERANCE
            && (self.open - other.open).abs() < TOLERANCE
            && (self.volume - other.volume).abs() < TOLERANCE
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Underlying {
    ask: f64,
    askSize: f64,
    bid: f64,
    bidSize: f64,
    change: f64,
    close: f64,
    delayed: bool,
    description: String,
    exchangeName: String,
    fiftyTwoWeekHigh: f64,
    fiftyTwoWeekLow: f64,
    highPrice: f64,
    pub last: f64,
    lowPrice: f64,
    mark: f64,
    markChange: f64,
    markPercentChange: f64,
    openPrice: f64,
    percentChange: f64,
    quoteTime: f64,
    symbol: String,
    totalVolume: f64,
    tradeTime: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionChainOption {
    putCall: String,
    symbol: String,
    description: String,
    exchangeName: String,
    // the next four are wrong bidPrice -> bid
    bid: f64,
    ask: f64,
    pub last: f64,
    mark: f64,
    bidSize: f64,
    askSize: f64,
    lastSize: f64,
    highPrice: f64,
    lowPrice: f64,
    openPrice: f64,
    closePrice: f64,
    totalVolume: f64,
    quoteTimeInLong: f64,
    tradeTimeInLong: f64,
    netChange: f64,
    volatility: f64,
    delta: f64,
    gamma: f64,
    theta: f64,
    vega: f64,
    rho: f64,
    timeValue: f64,
    openInterest: f64,
    // wrong isInTheMoney -> inTheMoney
    inTheMoney: bool,
    theoreticalOptionValue: f64,
    theoreticalVolatility: f64,
    //isMini
    mini: bool,
    //isNonStandard
    nonStandard: bool,
    optionDeliverablesList: Option<HashMap<String, String>>,
    strikePrice: f64,
    expirationDate: i64,
    expirationType: String,
    multiplier: f64,
    settlementType: String,
    deliverableNote: String,
    //isIndexOption
    indexOption: Option<bool>,
    percentChange: f64,
    markChange: f64,
    markPercentChange: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionChain {
    symbol: String,
    pub status: String,
    pub underlying: Underlying,
    // the below is sort of a lie
    strategy: String,
    interval: f64,
    isDelayed: bool,
    isIndex: bool,
    daysToExpiration: f64,
    interestRate: f64,
    underlyingPrice: f64,
    volatility: f64,
    // expiration date -> {strike price -> option}
    pub callExpDateMap: HashMap<String, HashMap<String, Vec<OptionChainOption>>>,
    putExpDateMap: HashMap<String, HashMap<String, Vec<OptionChainOption>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Price {
    dollars: i32,
    cents: i32,
}

impl<'de> Visitor<'de> for Price {
    type Value = Price;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a price that includes a decimal point")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let cents = (value * 100.0).abs().floor() as i32;
        Ok(Price{
            dollars: cents / 100,
            cents: cents % 100,
        })
    }

}