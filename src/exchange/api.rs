use super::super::reqwest;

use exchange::error::*;

pub trait Api {
    fn new() -> Self
    where
        Self: Sized + Send + Sync;
    fn ticker(&self, pair: (Coin, Coin)) -> Result<TradingTicker>;
    fn exchange_name(&self) -> String;
}

pub enum Coin {
    Bitcoin,
    Ethereum,
}

pub struct TradingTicker {
    pub last_highest_bid_price: f32,
    pub last_hightest_bid_size: f32,
    pub last_lowest_ask_price: f32,
    pub last_lowest_ask_size: f32,
    pub daily_change: f32,
    pub daily_change_percentage: f32,
    pub last_trade_price: f32,
    pub daily_volume: f32,
    pub daily_high: f32,
    pub daily_low: f32,
}


pub struct _FundingTicker {
    pub flash_return_rate: f32,
    pub last_highest_bid_price: f32,
    pub bid_period_days: i32,
    pub last_hightest_bid_size: f32,
    pub last_lowest_ask_price: f32,
    pub ask_period_days: i32,
    pub last_lowest_ask_size: f32,
    pub daily_change: f32,
    pub daily_change_percentage: f32,
    pub last_trade_price: f32,
    pub daily_volume: f32,
    pub daily_high: f32,
    pub daily_low: f32,
}
