use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CoinFile {
    pub coins: HashMap<String, Coin>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum Coin {
    Simple(String),
    Detailed(DetailedCoin),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetailedCoin {
    pub name: String,
    pub bitfinex: Option<String>,
    pub coinmarketcap: Option<CoinMarketCap>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum CoinMarketCap {
    Simple(String),
    Detailed(DetailedCoinMarketCap),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DetailedCoinMarketCap {
    id: String,
    conversion_symbol: String,
}
