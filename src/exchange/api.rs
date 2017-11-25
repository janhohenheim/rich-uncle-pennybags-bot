use super::super::reqwest;

use exchange::error::*;
use exchange::types::*;

pub struct Api {
    client: reqwest::Client,
}
impl Api {
    pub fn new() -> Self {
        Api {
            client: reqwest::Client::new(),
        }
    }
    pub fn ticker(&self, pair: &str) -> Result<TradingTicker> {
        let endpoint = format!("ticker/t{}", pair.to_uppercase());
        let response: Vec<f32> = self.make_request(&endpoint).send()?.json()?;
        parse_ticker(&response)
    }
    fn make_request(&self, endpoint: &str) -> reqwest::RequestBuilder {
        const API_URL_PREFIX: &str = "https://api.bitfinex.com/v2/";
        let url = format!("{}{}", API_URL_PREFIX, endpoint);
        self.client.get(&url)
    }
}

fn parse_ticker(v: &[f32]) -> Result<TradingTicker> {
    if v.len() != 10 {
        Err(Error::DeserializationError)
    } else {
        Ok(TradingTicker {
            last_highest_bid_price: v[0],
            last_hightest_bid_size: v[1],
            last_lowest_ask_price: v[2],
            last_lowest_ask_size: v[3],
            daily_change: v[4],
            daily_change_percentage: v[5],
            last_trade_price: v[6],
            daily_volume: v[7],
            daily_high: v[8],
            daily_low: v[9],
        })
    }
}
