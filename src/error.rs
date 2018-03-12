use std::convert;
use std::fmt;
use super::models::error::Error as ErrorModel;

#[derive(Debug)]
pub enum Error {
    UrlError(::reqwest::UrlError),
    HttpError(::reqwest::Error),
    ApiError(ErrorModel),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UrlError(ref err) => write!(f, "Failed when parsing the URL: {}", err),
            Error::HttpError(ref err) => write!(f, "Failed when executing the request: {}", err),
            Error::ApiError(ref err) => write!(f, "Airly returned an error: {}", err),
        }
    }
}

impl convert::From<::reqwest::UrlError> for Error {
    fn from(err: ::reqwest::UrlError) -> Self {
        Error::UrlError(err)
    }
}

impl convert::From<::reqwest::Error> for Error {
    fn from(err: ::reqwest::Error) -> Self {
        Error::HttpError(err)
    }
}