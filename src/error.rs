use super::rocket::error::LaunchError as BotError;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Telegram,
    Exchange,
    Bot(BotError),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Telegram => write!(f, "Telegram error"),
            Error::Exchange => write!(f, "Exchange error"),
            Error::Bot(ref err) => err.fmt(f),
        }
    }
}

use std::error::Error as StdError;
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Telegram => "Telegram error",
            Error::Exchange => "Exchange error",
            Error::Bot(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Telegram => None,
            Error::Exchange => None,
            Error::Bot(ref err) => Some(err),
        }
    }
}


impl From<BotError> for Error {
    fn from(err: BotError) -> Error {
        Error::Bot(err)
    }
}
