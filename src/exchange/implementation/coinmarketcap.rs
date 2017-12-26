use serde_json;
use reqwest;
use heck::KebabCase;

use self::serde_json::Value;
use exchange::error::*;
use exchange::*;
use model::Coin;
use model::toml::Name;
use model::toml::CoinMarketCap as CoinMarketCapName;

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

    fn ticker(&self, pair: &(Coin, Coin)) -> Result<TradingTicker> {
        let long_name = get_long_name(&pair.0);
        let conversion_symbol = get_converson_symbol(&pair.1);
        let endpoint = format!("ticker/{}/?convert={}", long_name, conversion_symbol);
        let response = self.make_request(&endpoint).send()?.text()?;
        let response: Value = serde_json::from_str(&response)?;
        let response = &response[0];

        let daily_change_percentage = parse_field(&response["percent_change_24h"])?;
        let field = format!("price_{}", conversion_symbol);
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
    fn make_request(&self, endpoint: &str) -> reqwest::RequestBuilder {
        const API_URL_PREFIX: &str = "https://api.coinmarketcap.com/v1/";
        let url = format!("{}{}", API_URL_PREFIX, endpoint);
        self.client.get(&url)
    }
}

fn get_long_name(coin: &Coin) -> String {
    match coin.name {
        Name::Simple(ref long_name) => &long_name,
        Name::Detailed(ref name) => match name.coinmarketcap {
            Some(ref coinmarketcap) => match coinmarketcap {
                &CoinMarketCapName::Simple(ref id) => id,
                &CoinMarketCapName::Detailed(ref coinmarketcap) => &coinmarketcap.id,
            },
            None => &name.long_name,
        },
    }.to_kebab_case()
}

fn get_converson_symbol(coin: &Coin) -> String {
    match coin.name {
        Name::Simple(_) => &coin.short_name,
        Name::Detailed(ref name) => match name.coinmarketcap {
            Some(ref coinmarketcap) => match coinmarketcap {
                &CoinMarketCapName::Simple(_) => &coin.short_name,
                &CoinMarketCapName::Detailed(ref coinmarketcap) => &coinmarketcap.conversion_symbol,
            },
            None => &coin.short_name,
        },
    }.to_lowercase()
}

fn parse_field(val: &Value) -> Result<f32> {
    val.as_str()
        .ok_or(Error::DeserializationError)?
        .parse::<f32>()
        .map_err(|_| Error::DeserializationError)
}
