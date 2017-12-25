extern crate serde_json;
use super::reqwest;

use self::serde_json::Value;
use exchange::error::*;
use exchange::*;

#[derive(Debug)]
pub struct CoinMarketCap {
    client: reqwest::Client,
}
impl Api for CoinMarketCap {
    fn new() -> Self {
        CoinMarketCap {
            client: reqwest::Client::new(),
        }
    }

    fn ticker(&self, pair: (Coin, Coin)) -> Result<TradingTicker> {
        let coins = Self::parse_coins(pair);
        let endpoint = format!("ticker/{}/?convert={}", coins.0, coins.1);
        let response = self.make_request(&endpoint).send()?.text()?;
        let response: Value = serde_json::from_str(&response)?;
        let response = &response[0];

        let daily_change_percentage = parse_field(&response["percent_change_24h"])?;
        let field = format!("price_{}", coins.1);
        let last_trade_price = parse_field(&response[&field])?;

        Ok(TradingTicker {
            daily_change_percentage,
            last_trade_price,
        })
    }

    fn exchange_name(&self) -> String {
        "💲 CoinMarketCap".into()
    }
}

impl CoinMarketCap {
    fn parse_coins(coins: (Coin, Coin)) -> (String, String) {
        (Self::parse_coin(coins.0), Self::parse_coin(coins.1))
    }

    fn parse_coin(coin: Coin) -> String {
        match coin {
            Coin::USDollar => "usd",
            Coin::Bitcoin => "bitcoin",
            Coin::Ethereum => "ethereum",
            Coin::Iota => "iota",
        }.to_string()
    }

    fn make_request(&self, endpoint: &str) -> reqwest::RequestBuilder {
        const API_URL_PREFIX: &str = "https://api.coinmarketcap.com/v1/";
        let url = format!("{}{}", API_URL_PREFIX, endpoint);
        self.client.get(&url)
    }
}

fn parse_field(val: &Value) -> Result<f32> {
    val.as_str()
        .ok_or(Error::DeserializationError)?
        .parse::<f32>()
        .map_err(|_| Error::DeserializationError)
}