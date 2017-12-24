use super::rocket::{self, State};
use super::rocket_contrib::Json;
use telegram::Api as TelegramApi;
use telegram::types::*;
use exchange::Api as ExchangeApi;
use exchange::implementation::Bitfinex;
use exchange::Coin;
use error::*;


type Exchanges = Vec<Box<ExchangeApi + Send + Sync>>;
type Exchange = Box<ExchangeApi + Send + Sync>;

#[post("/", data = "<update>")]
fn receive_update (
    update: Json<Update>,
    telegram: State<TelegramApi>,
    exchanges: State<Exchanges>,
) -> Result<()> {
    if let Some(ref message) = update.message {
        if let Some(ref text) = telegram.extract_text(&message) {
            if text.starts_with('/') {
                let text = &text[1..];
                let chat_id = message.chat.id;
                if text == "help" {
                    if handle_help(chat_id, &telegram).is_err() {
                        println!("Failed to send help");
                    }
                } else {
                    let mut coins = split_coins(&text);
                    if coins.len() == 1 {
                        coins.push("usd");
                    }
                    if coins.len() == 2 {
                        for exchange in exchanges.iter() {
                            let pair = (coins[0], coins[1]);
                            // try both combinations
                            if handle_pair(pair, chat_id, &telegram, exchange).is_err() {
                                let inverse = (pair.1, pair.0);
                                if handle_pair(inverse, chat_id, &telegram, exchange).is_err() {
                                    println!("Failed to answer to message: {}", text);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn parse_coins(symbols: (&str, &str)) -> Result<(Coin, Coin)> {
    Ok((parse_coin(symbols.0)?, parse_coin(symbols.1)?))
}

fn parse_coin(symbol: &str) -> Result<Coin> {
    match symbol {
        "btc" => Ok(Coin::Bitcoin),
        "eth" => Ok(Coin::Ethereum),
        _ => Err(Error::Parse(symbol.to_string())),
    }
}

fn handle_help(chat_id: i64, telegram: &TelegramApi) -> Result<()> {
    telegram.send_message(chat_id, "\
        Simply use /<coinpair>. If you only specify one coin, it assumes you want it in USD. Examples:\n\
        \n\
        /eth Returns the current ETH/USD rate\n\
        /ethbtc Returns the current ETH/BTC rate\n\
        \n\
        Available coins:\n\
        /btc\tBitcoin\n\
        /ltc\tLitecoin\n\
        /eth\tEthereum\n\
        /etc\tEthereum Classic\n\
        /zec\tZCash\n\
        /xmr\tMonero\n\
        /das\tDash\n\
        /xrp\tRipple\n\
        /iot\tIota\n\
        /eos\tEOS\n\
        /san\tSantiment\n\
        /omg\tOmiseGO\n\
        /bch\tBcash\n\
        /neo\tNEO\n\
        /etp\tETP\n\
        /qtu\tQtum\n\
        /avt\tAventus\n\
        /edo\tEidoo\n\
        /btg\tBTG\n\
        /dat\tStreamr\n\
        /rrt\tRecovery Right Tokens\n\
        \n\
        Please tell @Kekmeister if you want any additional features.\
    ")?;
    Ok(())
}

fn split_coins<'a>(text: &'a str) -> Vec<&'a str> {
    const COIN_LEN: usize = 3;
    if text.len() <= COIN_LEN {
        vec![text]
    } else {
        let (a, b) = text.split_at(COIN_LEN);
        let mut coins = split_coins(b);
        coins.push(a);
        coins
    }
}

fn handle_pair(coins: (&str, &str), chat_id: i64, telegram: &TelegramApi, exchange: &Exchange) -> Result<()> {
    let pair = parse_coins(coins)?;
    let ticker = exchange.ticker(pair)?;
    let exchange_name = format!("*{}*", exchange.exchange_name());
    
    let last_price =  ticker.last_trade_price;
    let mut price_amount = format!("{:.*}", 2, last_price);
    if price_amount == "0.00" {
        price_amount = format!("{:.*}", 6, last_price);
    }
    let price = format!(
        "{} {}/{}", 
        price_amount, 
        coins.0.to_uppercase(), 
        coins.1.to_uppercase()
    );

    let percentage = ticker.daily_change_percentage;
    let emoji = get_development_emoji(percentage);
    let development = format!(
        "{}{:.*}% in the last 24h",
         emoji, 
         2, percentage * 100.0);

    let msg = format!("{}\n{}\n{}", exchange_name, price, development);
    telegram.send_message(chat_id, &msg)?;
    Ok(())
}

fn get_development_emoji(percentage: f32) -> &'static str {
    if percentage.is_sign_positive() {
        "📈 +"
    } else {
        "📉 "
    }
}

pub struct RichUnclePennybagsBot {
    telegram: TelegramApi,
    exchanges: Exchanges,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str, username: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token, username),
            exchanges: vec![Box::new(Bitfinex::new())],
        }
    }
    pub fn start(self) -> Error {
        rocket::ignite()
            .manage(self.telegram)
            .manage(self.exchanges)
            .mount("/", routes![receive_update])
            .launch()
            .into()
    }
}
