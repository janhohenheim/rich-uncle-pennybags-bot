use super::serde::{self, de};
use serde::Deserialize;
use std::fmt;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug)]
pub struct Coin {
    pub name: String,
    pub exchanges: Option<Vec<Exchange>>,
}

#[derive(Deserialize, Debug)]
pub struct Exchange {
    pub bitfinex: Option<String>,
    pub coinmarketcap: Option<BTreeMap<String, CoinMarketCapConf>>,
 }

#[derive(Debug)]
#[serde(untagged)]
pub enum CoinMarketCapConf {
    Simple(String),
    Detailed(DetailedCoinMarketCapConf)
}

impl<'de> de::Deserialize<'de> for CoinMarketCapConf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: de::Deserializer<'de>
    {
        struct CoinMarketCapConfVisitor;

        impl<'de> de::Visitor<'de> for CoinMarketCapConfVisitor {
            type Value = CoinMarketCapConf;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("A coin id like \"ethereum\" or a detailed config \
                like { id = \"ethereum\", conversion_symbol = \"eth\"}")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(CoinMarketCapConf::Simple(s.to_owned()))
            }

            fn visit_map<V>(self, map: V) -> Result<Self::Value, V::Error>
                where V: de::MapAccess<'de>
            {
                let mvd = de::value::MapAccessDeserializer::new(map);
                DetailedCoinMarketCapConf::deserialize(mvd).map(CoinMarketCapConf::Detailed)
            }
        }

        deserializer.deserialize_any(CoinMarketCapConfVisitor)
    }
}

#[derive(Deserialize, Debug)]
pub struct DetailedCoinMarketCapConf {
    id: String,
    conversion_symbol: String,
}