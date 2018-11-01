#[derive(Clone, Debug)]
pub struct RateLimit {
    // Number of requests (per API key) you can execute each day.
    pub daily_limit: u32,

    // Number of requests (per API key) you can still execute today.
    // Exceeding this limit (getting it below 0) will result in HTTP 429 errors.
    pub daily_remaining: u32,

    // Number of requests (per API key) you can execute each minute.
    pub minute_limit: u32,

    // Number of request (per API key) you can still execute in this minute.
    // Exceeding this limit (getting it below 0) will result in HTTP 429 errors.
    pub minute_remaining: u32,
}
