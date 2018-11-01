use std::result;
use utils::Error;

pub type Result<T> = result::Result<T, Error>;
