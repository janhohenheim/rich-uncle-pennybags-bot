use rocket::State;
use rocket_contrib::Json;

use telegram::Api as TelegramApi;
use telegram::model::*;
use exchange::Api as ExchangeApi;
use model::Coin;
use error::*;

type Exchanges = Vec<Exchange>;
type Exchange = Box<ExchangeApi + Send + Sync>;

#[post("/", data = "<update>")]
pub fn receive_update(
    update: Json<Update>,
    telegram: State<TelegramApi>,
    exchanges: State<Exchanges>,
    coins: State<Vec<Coin>>,
) -> Result<()> {
    if let Some(ref message) = update.message {
        if let Some(ref text) = telegram.extract_text(&message) {
            if text.starts_with('/') {
                let command = text[1..].to_lowercase();
                let chat_id = message.chat.id;
                let command_handler = CommandHandler {
                    command: &command,
                    chat_id,
                    coins: &coins,
                    telegram: &telegram,
                    exchanges: &exchanges,
                };
                match command.as_ref() {
                    "help" => handle_help(&coins, chat_id, &telegram)?,
                    "ens" => command_handler.handle_ens()?,
                    _ => command_handler.handle_ticker()?,
                }
            }
        }
    }
    Ok(())
}

struct CommandHandler<'a> {
    command: &'a str,
    chat_id: i64,
    coins: &'a [Coin],
    telegram: &'a TelegramApi,
    exchanges: &'a Exchanges,
}

impl<'a> CommandHandler<'a> {
    fn send_message(&self, msg: &str) -> Result<Response<Message>> {
        self.telegram
            .send_message(self.chat_id, msg)
            .map_err(|e| e.into())
    }

    fn handle_ticker(&self) -> Result<()> {
        let mut symbols: Vec<_> = self.command.split('_').collect();
        if symbols.len() == 1 {
            symbols.push("usd");
        }
        if symbols.len() == 2 {
            let first_coin = parse_coin(self.coins, symbols[0]);
            if let Err(e) = first_coin {
                println!(
                    "Failed to parse first coin: \"{}\", error: {}",
                    symbols[0], e
                );
                return Ok(());
            }
            let first_coin = first_coin.unwrap();
            let second_coin = parse_coin(self.coins, symbols[1]);
            if let Err(e) = second_coin {
                println!(
                    "Failed to parse second coin: \"{}\", error: {}",
                    symbols[1], e
                );
                return Ok(());
            }
            let second_coin = second_coin.unwrap();
            let pair = (first_coin, second_coin);
            let inverse = (pair.1.clone(), pair.0.clone());
            for exchange in self.exchanges.iter() {
                // try both combinations
                if handle_pair(&pair, self.chat_id, self.telegram, exchange).is_err() {
                    if let Err(err) = handle_pair(&inverse, self.chat_id, self.telegram, exchange) {
                        println!(
                            "Failed to answer to message: {}, error: {:?}",
                            self.command, err
                        );
                    }
                }
            }
        }
        Ok(())
    }
    fn handle_ens(&self) -> Result<()> {
        self.send_message("[Ethereum Name Service](https://www.myetherwallet.com/#ens)")
            .map(|_| ())
    }
}

fn parse_coin(coins: &[Coin], symbol: &str) -> Result<Coin> {
    coins
        .iter()
        .find(|coin| coin.short_name == symbol)
        .map(|coin| coin.clone())
        .ok_or(Error::Parse(symbol.to_string()))
}

fn handle_help(coins: &Vec<Coin>, chat_id: i64, telegram: &TelegramApi) -> Result<()> {
    let mut msg = "
Simply use `/firstcoin_secondcoin`. If you only specify one coin, the bot assumes you want it in USD. Examples:

`/eth` Returns the current Ethereum to U.S. Dollar rate
`/eth_btc` Returns the current Ethereum to Bitcoin rate
`/btc_chf` Returns the current Bitcoin to Swiss Franc rate

Available currencies:
"
        .to_string();
    let footer = "
You can add a new currency yourself by adding it to the [coinfile](https://github.com/SirRade/rich-uncle-pennybags-bot/blob/master/Coins.toml)
Please tell @Kekmeister if you want any additional features.

If you're german speaking, feel free to [join us](https://t.me/joinchat/Azh980Rug594nvfzLEQsIw) 🙂
Last but not least, if you enjoy the bot consider buying me a drink at [jnferner.eth (0x74cc5Ee15E0D13Da72d459a8166e61897E4C308D)](https://etherscan.io/address/0x74cc5Ee15E0D13Da72d459a8166e61897E4C308D)";
    for coin in coins {
        let long_name = coin.long_name();
        let command = format!("- {} _{}_\n", coin.short_name, long_name);
        msg.push_str(&command)
    }
    msg.push_str(footer);
    telegram.send_message(chat_id, &msg)?;
    Ok(())
}

fn handle_pair(
    pair: &(Coin, Coin),
    chat_id: i64,
    telegram: &TelegramApi,
    exchange: &Exchange,
) -> Result<()> {
    let ticker = exchange.ticker(pair)?;
    let exchange_name = format!("*{}*", exchange.exchange_name());

    let long_names = (pair.0.long_name(), pair.1.long_name());

    let last_price = ticker.last_trade_price;
    let mut price_amount = format!("{:.*}", 2, last_price);
    if price_amount == "0.00" {
        price_amount = format!("{:.*}", 6, last_price);
    }
    let price = format!(
        "{} {}/{}",
        price_amount,
        pair.0.short_name.to_uppercase(),
        pair.1.short_name.to_uppercase()
    );

    let percentage = ticker.daily_change_percentage;
    let emoji = development_emoji(percentage);
    let development = format!("{}{:.*}% in the last 24h", emoji, 2, percentage);

    let msg = format!(
        "{}\n{} - {}\n{}\n{}",
        exchange_name, long_names.0, long_names.1, price, development
    );
    telegram.send_message(chat_id, &msg)?;
    Ok(())
}

fn development_emoji(percentage: f32) -> &'static str {
    if percentage.is_sign_positive() {
        "📈 +"
    } else {
        "📉 "
    }
}
