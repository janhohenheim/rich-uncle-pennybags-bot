extern crate reqwest;

pub mod bitfinex;
pub mod coinmarketcap;

pub use self::bitfinex::Bitfinex;
pub use self::coinmarketcap::CoinMarketCap;
