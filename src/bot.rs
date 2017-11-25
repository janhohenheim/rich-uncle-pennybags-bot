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
    // Todo: Completely rewrite this shit method
    let post = if let Some(ref post) = update.message {
        post
    } else if let Some(ref post) = update.channel_post {
        post
    } else {
        panic!();
    };
    let msg = &post.text.to_lowercase();
    let id = post.chat.id;

    let usd = |coin: &str| -> Result<()> {
        if msg == &format!("/{}", coin) 
        || msg == &format!("/{}usd", coin)
        || msg == &format!("/usd{}", coin) 
        || msg == &format!("/{}@RichUnclePennybagsBot", coin)
        || msg == &format!("/{}usd@RichUnclePennybagsBot", coin) 
        || msg == &format!("/usd{}@RichUnclePennybagsBot", coin){
            let pair = format!("{}usd", coin);
            let ticker = exchange.ticker(&pair)?;
            let name = exchange.exchange_name();
            let percentage = ticker.daily_change_percentage;
            let emoji = 
            if percentage.is_sign_positive() {
                "📈 +"
            } else {
                "📉 "
            };
            let msg = format!(
                "*{}*\n{:.*} USD/{}\n{}{}% in the last 24h",
                name,
                2,
                ticker.last_trade_price,
                coin.to_uppercase(),
                emoji,
                percentage * 100.0,
            );
            telegram.send_message(id, &msg)?;
        }
        Ok(())
    };
    usd("eth")?;
    usd("iot")?;
    usd("btc")?;
    usd("omg")?;
    Ok(())
}

pub struct RichUnclePennybagsBot {
    telegram: TelegramApi,
    exchange: ExchangeApi,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token),
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
