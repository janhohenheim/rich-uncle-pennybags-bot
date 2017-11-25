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
    let msg = &update.message.text.to_lowercase();
    let id = update.message.chat.id;

    let usd = |coin: &str| -> Result<()> {
        if msg == &format!("/{}", coin) {
            let pair = format!("{}usd", coin);
            let ticker = exchange.ticker(&pair)?;
            let msg = format!("{:.*}$", 2, ticker.last_trade_price);
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
