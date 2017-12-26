use rocket::State;
use rocket_contrib::Json;

use telegram::Api as TelegramApi;
use telegram::types::*;
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
                let text = &text[1..];
                let chat_id = message.chat.id;
                if text == "help" {
                    if handle_help(chat_id, &telegram).is_err() {
                        println!("Failed to send help");
                    }
                } else {
                    let mut symbols = split_coins(&text);
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
