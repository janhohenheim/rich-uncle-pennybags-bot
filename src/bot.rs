use super::rocket;
use super::toml;

use telegram::Api as TelegramApi;
use exchange::Api as ExchangeApi;
use exchange::implementation::*;
use error::*;
use model::*;
use routes;


type Exchanges = Vec<Exchange>;
type Exchange = Box<ExchangeApi + Send + Sync>;



pub struct RichUnclePennybagsBot {
    telegram: TelegramApi,
    exchanges: Exchanges,
    //coins: Vec<Coin>,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str, username: &str, coinfile: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token, username),
            exchanges: vec![
                Box::new(Bitfinex::new()),
                Box::new(CoinMarketCap::new()),
            ],
            //coins: parse_coins(coinfile)
        }
    }
    pub fn start(self) -> Error {
        rocket::ignite()
            .manage(self.telegram)
            .manage(self.exchanges)
            //.manage(self.coins)
            .mount("/", routes![routes::receive_update])
            .launch()
            .into()
    }
}

fn parse_coins(coinfile: &str) -> Vec<Coin> {
        use std::fs::File;
        use std::io::Read;

        const ERROR_MSG: &str =
"Failed to parse coinfile. 
It either contains invalid TOML or a required key has been ommited";

        let mut cointoml = String::new();
        File::open(coinfile)
            .expect("Failed to open coinfile")
            .read_to_string(&mut cointoml)
            .expect("Failed to read coinfile");
        toml::from_str(&cointoml).expect(ERROR_MSG)
}