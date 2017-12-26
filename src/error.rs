use rocket::error::LaunchError;

use std::result;
use telegram::error::Error as TelegramError;
use exchange::error::Error as ExchangeError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Telegram(TelegramError),
    Exchange(ExchangeError),
    Server(LaunchError),
    Parse(String),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Telegram(ref err) => err.fmt(f),
            Error::Exchange(ref err) => err.fmt(f),
            Error::Server(ref err) => err.fmt(f),
            Error::Parse(ref err) => {
                write!(f, "Couldn't match symbol '{}' with any supported coin", err)
            }
        }
    }
}

use std::error::Error as StdError;
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Telegram(ref err) => err.description(),
            Error::Exchange(ref err) => err.description(),
            Error::Server(ref err) => err.description(),
            Error::Parse(_) => "Failed to parse coin",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Telegram(ref err) => Some(err),
            Error::Exchange(ref err) => Some(err),
            Error::Server(ref err) => Some(err),
            Error::Parse(_) => None,
        }
    }
}

impl From<LaunchError> for Error {
    fn from(err: LaunchError) -> Error {
        Error::Server(err)
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
