use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CoinFile {
    pub coins: HashMap<String, CoinName>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CoinName {
    Simple(String),
    Detailed(DetailedCoinName),
}

#[derive(Deserialize, Debug)]
pub struct DetailedCoinName {
    pub name: String,
    pub bitfinex: Option<String>,
    pub coinmarketcap: Option<CoinMarketCapConf>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum CoinMarketCapConf {
    Simple(String),
    Detailed(DetailedCoinMarketCapConf),
}

#[derive(Deserialize, Debug)]
pub struct DetailedCoinMarketCapConf {
    id: String,
    conversion_symbol: String,
}
