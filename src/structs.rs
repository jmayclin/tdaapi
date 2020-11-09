extern crate serde;

use serde::Deserialize

#[derive(Deserialize, Debug)]
struct PostAccessToken {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: u32,
    scope: String,
    refresh_token_expires_in: u32
}


#[derive(Deserialize, Debug)]
struct QuoteStock {
//Equity:
    symbol: String,
    description: String,
    bidPrice: f64,
    bidSize: f64,
    bidId: String,
    askPrice: f64,
    askSize: f64,
    askId: String,
    lastPrice: f64,
    lastSize: f64,
    lastId: String,
    openPrice: f64,
    highPrice: f64,
    lowPrice: f64,
    closePrice: f64,
    netChange: f64,
    totalVolume: f64,
    quoteTimeInLong: f64,
    tradeTimeInLong: f64,
    mark: f64,
    exchange: String,
    exchangeName: String,
    marginable: false,
    shortable: false,
    volatility: f64,
    digits: f64,
    52WkHigh: f64,
    52WkLow: f64,
    peRatio: f64,
    divAmount: f64,
    divYield: f64,
    divDate: String,
    securityStatus: String,
    regularMarketLastPrice: f64,
    regularMarketLastSize: f64,
    regularMarketNetChange: f64,
    regularMarketTradeTimeInLong: f64
}

#[derive(Deserialize, Debug)]
struct QuoteOption {
  symbol: String,
  description: String,
  bidPrice: f64,
  bidSize: f64,
  askPrice: f64,
  askSize: f64,
  lastPrice: f64,
  lastSize: f64,
  openPrice: f64,
  highPrice: f64,
  lowPrice: f64,
  closePrice: f64,
  netChange: f64,
  totalVolume: f64,
  quoteTimeInLong: f64,
  tradeTimeInLong: f64,
  mark: f64,
  openInterest: f64,
  volatility: f64,
  moneyIntrinsicValue: f64,
  multiplier: f64,
  strikePrice: f64,
  contractType: String,
  underlying: String,
  timeValue: f64,
  deliverables: String,
  delta: f64,
  gamma: f64,
  theta: f64,
  vega: f64,
  rho: f64,
  securityStatus: String,
  theoreticalOptionValue: f64,
  underlyingPrice: f64,
  uvExpirationType: String,
  exchange: String,
  exchangeName: String,
  settlementType: String
}