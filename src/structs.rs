use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
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
    pub quoteTimeInLong: f64,
    pub tradeTimeInLong: f64,
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
    pub quoteTimeInLong: f64,
    pub tradeTimeInLong: f64,
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
    pub close: f64,
    pub datetime: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: f64,
}