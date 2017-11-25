use super::super::reqwest::Error as RequestError;

use std::result;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestError(RequestError),
}

use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RequestError(ref err) => err.fmt(f),
        }
    }
}

use std::error::Error as StdError;
impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::RequestError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::RequestError(ref err) => Some(err),
        }
    }
}


impl From<RequestError> for Error {
    fn from(err: RequestError) -> Error {
        Error::RequestError(err)
    }
}
