use rocket::State;
use rocket_contrib::Json;

use telegram::Api as TelegramApi;
use telegram::types::*;
use exchange::Api as ExchangeApi;
use model::Coin;
use model::toml::Name;
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
                let text = &text[1..];
                let chat_id = message.chat.id;
                if text == "help" {
                    if handle_help(&coins, chat_id, &telegram).is_err() {
                        println!("Failed to send help");
                    }
                } else {
                    let mut symbols: Vec<_> = text.split('_').collect();
                    if symbols.len() == 1 {
                        symbols.push("usd");
                    }
                    if symbols.len() == 2 {
                        let pair = (
                            parse_coin(&coins, symbols[0])?,
                            parse_coin(&coins, symbols[1])?,
                        );
                        let inverse = (pair.1.clone(), pair.0.clone());
                        for exchange in exchanges.iter() {
                            // try both combinations
                            if handle_pair(&pair, chat_id, &telegram, exchange).is_err() {
                                if let Err(err) =
                                    handle_pair(&inverse, chat_id, &telegram, exchange)
                                {
                                    println!(
                                        "Failed to answer to message: {}, error: {:?}",
                                        text, err
                                    );
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

fn parse_coin(coins: &Vec<Coin>, symbol: &str) -> Result<Coin> {
    coins
        .iter()
        .find(|coin| coin.short_name == symbol)
        .map(|coin| coin.clone())
        .ok_or(Error::Parse(symbol.to_string()))
}

fn handle_help(coins: &Vec<Coin>, chat_id: i64, telegram: &TelegramApi) -> Result<()> {
    let mut msg = "
Simply use /coin1_coin2. If you only specify one coin, the bot assumes you want it in USD. Examples:

/eth Returns the current Ethereum to U.S. Dollar rate
/eth_btc Returns the current Ethereum to Bitcoin rate
/btc_chf Returns the current Bitcoin to Swiss Franc rate

Available currencies:
"
        .to_string();
    let footer = "
You can add a new currency yourself by adding it to the [coinfile](https://github.com/SirRade/rich-uncle-pennybags-bot/blob/master/Coins.toml)
Please tell @Kekmeister if you want any additional features.";
    for coin in coins {
        let long_name = match coin.name {
            Name::Simple(ref long_name) => &long_name,
            Name::Detailed(ref detailed_name) => &detailed_name.long_name,
        };
        let command = format!("{}: {}\n", coin.short_name, long_name);
        msg.push_str(&command)
    }
    msg.push_str(footer);
    println!("msg:{}", msg);
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
    let emoji = get_development_emoji(percentage);
    let development = format!("{}{:.*}% in the last 24h", emoji, 2, percentage);

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
