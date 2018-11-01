use reqwest;
use std::error;
use std::fmt;
use utils::ErrorContext;

#[derive(Debug)]
pub enum Error {
    /// Given API key is invalid / expired.
    InvalidCredentials(ErrorContext),

    /// Response had an unexpected or invalid format.
    InvalidResponse(Box<reqwest::Error>),

    /// You've reached limit for this minute / day / IP address.
    RateLimitReached(ErrorContext),

    /// You've requested a resource that does not exist (e.g. an old installation).
    ResourceNotFound(ErrorContext),

    /// Response had an unexpected status code.
    UnexpectedHttpStatusCode(u16),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::InvalidResponse(Box::new(error))
    }
}

impl error::Error for Error {
    // Nottin' here - it functions as a marker only
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidCredentials(_) => write!(f, "Given API key is invalid / expired."),
            Error::InvalidResponse(error) => write!(f, "Response had an unexpected or invalid format: {}", error),
            Error::RateLimitReached(_) => write!(f, "You've reached limit for this minute / day / IP address - please try again later."),
            Error::ResourceNotFound(_) => write!(f, "You've requested a resource that does not exist (e.g. an old installation)."),
            Error::UnexpectedHttpStatusCode(code) => write!(f, "Response had an unexpected status code: [{}].", code),
        }
    }
}
