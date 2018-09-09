use super::Result;

airly_model! {
    pub struct ErrorContext {
        pub error_code: Option<String>,
        pub message: String,
    }
}

pub struct Response<T> {
    result: Result<T>,
    rate_limit: RateLimitInfo,
}

pub struct RateLimitInfo {
    // Number of requests (per API key) you can execute each day.
    pub daily_limit: u32,

    // Number of requests (per API key) you can still execute today.
    // Exceeding this limit (getting it below 0) will result in server returning HTTP 429 responses.
    pub daily_remaining: u32,

    // Number of requests (per API key) you can execute each minute.
    pub minute_limit: u32,

    // Number of request (per API key) you can still execute in this minute.
    // Exceeding this limit (getting it below 0) will result in server returning HTTP 429 responses.
    pub minute_remaining: u32,
}
