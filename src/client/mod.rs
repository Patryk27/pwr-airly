pub use self::client::*;
pub use self::models::*;
pub use self::result::*;

header! { (ApiKey, "ApiKey") => [String] }

pub(crate) const API_URL: &str = "https://airapi.airly.eu/v2";

mod client;
mod models;
mod result;
