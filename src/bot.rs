use super::rocket::{self, State};
use super::rocket_contrib::Json;
use error::*;
use telegram::Api as TelegramApi;
use telegram::types::*;
use exchange::*;

#[post("/", data = "<update>")]
fn receive_update(update: Json<Update>, telegram: State<TelegramApi>) -> Result<String> {
    telegram.send_message(update.message.chat.id, &update.message.text)?;
    Ok("Ok".to_string())
}
pub struct RichUnclePennybagsBot {
    telegram: TelegramApi,
    exchange: ExchangeApi,
}
impl RichUnclePennybagsBot {
    pub fn new(token: &str) -> Self {
        RichUnclePennybagsBot {
            telegram: TelegramApi::new(token),
            exchange: ExchangeApi {},
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
