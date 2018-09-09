use reqwest;
use std::result;
use super::ErrorContext;

#[derive(Debug)]
pub enum Error {
    // Returned when user's API key is invalid / expired.
    InvalidKey(ErrorContext),

    // Returned when user's requested a resource that does not exist, e.g.: an old installation.
    ResourceNotFound(ErrorContext),

    // Returned when user's overburdened Airly's API with too many requests.
    TooManyRequests(ErrorContext),

    // Returned when there has been an unknown / internal error, e.g.: returned JSON had invalid
    // format.
    Unknown(String),
}

pub type Result<T> = result::Result<T, Error>;

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Unknown(
            format!("Failed to parse the response's JSON - this shouldn't happen. Error message: {}", error)
        )
    }
}
