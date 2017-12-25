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
        let coins = Self::parse_coins(pair)?;
        let pair = format!("{}{}", coins.0, coins.1);
        let endpoint = format!("ticker/t{}", pair.to_uppercase());
        let response: Vec<f32> = self.make_request(&endpoint).send()?.json()?;
        parse_ticker(&response)
    }

    fn exchange_name(&self) -> String {
        "🍃 Bitfinex".into()
    }
}

impl Bitfinex {
    fn parse_coins(coins: (Coin, Coin)) -> Result<(String, String)> {
        Ok((Self::parse_coin(coins.0)?, Self::parse_coin(coins.1)?))
    }

    fn parse_coin(coin: Coin) -> Result<String> {
        Ok(
            match coin {
                Coin::USDollar => "usd",
                Coin::Bitcoin => "btc",
                Coin::Ethereum => "eth",
                Coin::Iota => "iot",

                Coin::RequestNetwork => return Err(Error::CoinNotSupported(coin)),
            }.to_string()
        )
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
            daily_change_percentage: v[5] * 100.0,
            last_trade_price: v[6],
        })
    }
}
