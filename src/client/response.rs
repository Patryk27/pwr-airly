use models::Model;

#[derive(Clone, Debug)]
pub struct Response<T> {
    model: T,
    rate_limit: RateLimit,
}

impl<T: Model> Response<T> {
    pub fn new(model: T, rate_limit: RateLimit) -> Self {
        Self {
            model,
            rate_limit,
        }
    }

    pub fn model(&self) -> T {
        self.model.clone()
    }

    pub fn rate_limit(&self) -> RateLimit {
        self.rate_limit.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RateLimit {
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
