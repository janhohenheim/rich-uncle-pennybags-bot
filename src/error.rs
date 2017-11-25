use super::rocket::error::LaunchError;

use std::result;
use telegram::error::Error as TelegramError;
use exchange::error::Error as ExchangeError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Telegram(TelegramError),
    Exchange(ExchangeError),
    Bot(LaunchError),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Telegram(ref err) => err.fmt(f),
            Error::Exchange(ref err) => err.fmt(f),
            Error::Bot(ref err) => err.fmt(f),
        }
    }
}

use std::error::Error as StdError;
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Telegram(ref err) => err.description(),
            Error::Exchange(ref err) => err.description(),
            Error::Bot(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Telegram(ref err) => Some(err),
            Error::Exchange(ref err) => Some(err),
            Error::Bot(ref err) => Some(err),
        }
    }
}


impl From<LaunchError> for Error {
    fn from(err: LaunchError) -> Error {
        Error::Bot(err)
    }
}


impl From<TelegramError> for Error {
    fn from(err: TelegramError) -> Error {
        Error::Telegram(err)
    }
}

impl From<ExchangeError> for Error {
    fn from(err: ExchangeError) -> Error {
        Error::Exchange(err)
    }
}
