#[derive(Deserialize)]
pub struct Coin {
    pub name: String,
    pub exchanges: Option<Vec<Exchange>>,
}

#[derive(Deserialize)]
pub struct Exchange {
    pub bitfinex: Option<String>,
    pub coinmarketcap: Option<String>,
}
