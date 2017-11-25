use super::rocket;
use error::*;
use telegram::*;
use exchange::*;

#[get("/")]
fn index() -> &'static str {
    "Hello, world! :>"
}

pub struct RichUnclePennybagsBot<'a> {
    _telegram: Telegram<'a>,
    _exchange: Exchange,
}
impl<'a> RichUnclePennybagsBot<'a> {
    pub fn new(token: &'a str) -> Self {
        RichUnclePennybagsBot {
            _telegram: Telegram::new(token),
            _exchange: Exchange {},
        }
    }
    pub fn start(self) -> Error {
        rocket::ignite().mount("/", routes![index]).launch().into()
    }
}
