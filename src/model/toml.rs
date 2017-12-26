use std::collections::HashMap;

#[derive(Deserialize, Debug, Cloneç)]
#[serde(deny_unknown_fields)]
pub struct CoinFile {
    pub coins: HashMap<String, Name>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum Name {
    Simple(String),
    Detailed(DetailedName),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DetailedName {
    pub long_name: String,
    pub bitfinex: Option<String>,
    pub coinmarketcap: Option<CoinMarketCap>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged, deny_unknown_fields)]
pub enum CoinMarketCap {
    Simple(String),
    Detailed(DetailedCoinMarketCap),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DetailedCoinMarketCap {
    pub id: String,
    pub conversion_symbol: String,
}
