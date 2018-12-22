use std::result;

use crate::client::Error;

pub type Result<T> = result::Result<T, Error>;
