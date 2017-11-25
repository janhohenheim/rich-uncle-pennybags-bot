use super::rocket::{self, State};
use super::rocket_contrib::Json;
use error::*;
use telegram::Api as TelegramApi;
use telegram::types::*;
use exchange::Api as ExchangeApi;

#[post("/", data = "<update>")]
fn receive_update(
    update: Json<Update>,
    telegram: State<TelegramApi>,
    exchange: State<ExchangeApi>,
) -> Result<()> {
    if let Some(ref message) = update.message {
        if let Some(ref text) = telegram.extract_text(&message) {
            if text.starts_with('/') {
                let text = text[1..].to_uppercase();
                let mut coins = split_coins(&text);
                if coins.len() == 1 {
                    coins.push("usd");
                }
                if coins.len() == 2 {
                    let pair = (coins[0], coins[1]);
                    let id = message.chat.id;
                    // try both combinations
                    if handle_pair(pair, id, &telegram, &exchange).is_err() {
                        let inverse = (pair.1, pair.0);
                        if handle_pair(inverse, id, &telegram, &exchange).is_err() {
                            println!("Failed to answer to message: {}", text);
                        }
                    }
                }
            }
        }
    }

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

fn handle_pair(coins: (&str, &str), chat_id: i64, telegram: &TelegramApi, exchange: &ExchangeApi) -> Result<()> {
    let ticker = exchange.ticker(coins)?;
    let exchange_name = format!("*{}*", exchange.exchange_name());
    
    let price = format!(
        "{:.*} {}/{}", 
        2, ticker.last_trade_price, 
        coins.0, coins.1);

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
    exchange: ExchangeApi,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str, username: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token, username),
            exchange: ExchangeApi::new(),
        }
    }
    pub fn start(self) -> Error {
        rocket::ignite()
            .manage(self.telegram)
            .manage(self.exchange)
            .mount("/", routes![receive_update])
            .launch()
            .into()
    }
}
