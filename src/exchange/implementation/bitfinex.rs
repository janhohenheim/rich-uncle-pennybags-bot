use super::reqwest;

use exchange::error::*;
use exchange::*;

#[derive(Debug)]
pub struct Bitfinex {
    client: reqwest::Client,
}
impl Api for Bitfinex {
    fn new() -> Self {
        Bitfinex {
            client: reqwest::Client::new(),
        }
    }
    fn ticker(&self, pair: (Coin, Coin)) -> Result<TradingTicker> {
        let coins = Self::parse_coins(pair);
        let pair = format!("{}{}", coins.0, coins.1);
        let endpoint = format!("ticker/t{}", pair.to_uppercase());
        let response: Vec<f32> = self.make_request(&endpoint).send()?.json()?;
        parse_ticker(&response)
    }

    fn exchange_name(&self) -> String {
        "Bitfinex 🍃".into()
    }
}

impl Bitfinex {
    fn parse_coins(coins: (Coin, Coin)) -> (String, String) {
        (Self::parse_coin(coins.0), Self::parse_coin(coins.1))
    }

    fn parse_coin(coin: Coin) -> String {
        match coin {
            Coin::USDollar => "usd",
            Coin::Bitcoin => "btc",
            Coin::Ethereum => "eth",
        }.to_string()
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
