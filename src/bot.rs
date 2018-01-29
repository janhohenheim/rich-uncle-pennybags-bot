use rocket;
use toml;

use telegram::Api as TelegramApi;
use exchange::Api as ExchangeApi;
use exchange::implementation::*;
use error::*;
use routes;
use model::Coin;
use model::toml::CoinFile;

type Exchanges = Vec<Exchange>;
type Exchange = Box<ExchangeApi + Send + Sync>;

pub struct RichUnclePennybagsBot {
    telegram: TelegramApi,
    exchanges: Exchanges,
    coins: Vec<Coin>,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str, username: &str, coinfile: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token, username),
            exchanges: vec![Box::new(Bitfinex::new()), Box::new(CoinMarketCap::new())],
            coins: parse_coins(coinfile),
        }
    }
    pub fn start(self) -> Error {
        rocket::ignite()
            .manage(self.telegram)
            .manage(self.exchanges)
            .manage(self.coins)
            .mount("/", routes![routes::receive_update])
            .launch()
            .into()
    }
}

fn parse_coins(coinfile: &str) -> Vec<Coin> {
    use std::fs::File;
    use std::io::Read;

    let error_msg = format!(
        "
Failed to parse coinfile at \"{}\". It either  
- contains invalid TOML
- a required key has been ommited
- some key has not been recognized (Did you make a typo?)

Exact error",
        coinfile
    );

    let mut cointoml = String::new();
    File::open(coinfile)
        .expect("Failed to open coinfile")
        .read_to_string(&mut cointoml)
        .expect("Failed to read coinfile");
    let coinfile: CoinFile = toml::from_str(&cointoml).expect(&error_msg);
    coinfile
        .coins
        .into_iter()
        .map(|(short_name, name)| Coin { short_name, name })
        .collect()
}
