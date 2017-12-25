use exchange::error::*;

pub trait Api {
    fn new() -> Self
    where
        Self: Sized + Send + Sync;
    fn ticker(&self, pair: (Coin, Coin)) -> Result<TradingTicker>;
    fn exchange_name(&self) -> String;
}

#[derive(Debug)]
pub enum Coin {
    USDollar,
    Bitcoin,
    Ethereum,
    Iota,

    RequestNetwork,
}

#[derive(Debug)]
pub struct TradingTicker {
    pub daily_change_percentage: f32,
    pub last_trade_price: f32,
}
