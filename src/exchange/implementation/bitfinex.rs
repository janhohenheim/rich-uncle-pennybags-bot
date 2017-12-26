use reqwest;

use exchange::error::*;
use exchange::*;
use model::Coin;
use model::toml::Name;

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
    fn ticker(&self, pair: &(Coin, Coin)) -> Result<TradingTicker> {
        let symbols = (get_symbol(&pair.0), get_symbol(&pair.1));
        let symbol_pair = format!("{}{}", symbols.0, symbols.1);
        let endpoint = format!("ticker/t{}", symbol_pair);
        let response: Vec<f32> = self.make_request(&endpoint).send()?.json()?;
        parse_ticker(&response)
    }

    fn exchange_name(&self) -> String {
        "🍃 Bitfinex".into()
    }
}

impl Bitfinex {
    fn make_request(&self, endpoint: &str) -> reqwest::RequestBuilder {
        const API_URL_PREFIX: &str = "https://api.bitfinex.com/v2/";
        let url = format!("{}{}", API_URL_PREFIX, endpoint);
        self.client.get(&url)
    }
}

fn get_symbol(coin: &Coin) -> String {
    match coin.name {
        Name::Simple(_) => &coin.short_name,
        Name::Detailed(ref name) => match name.bitfinex {
            Some(ref symbol) => symbol,
            None => &coin.short_name,
        },
    }.to_uppercase()
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
