use crate::client::RateLimit;
use crate::models::Model;

#[derive(Clone, Debug)]
pub struct Response<T: Model> {
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

    pub fn model(&self) -> &T {
        &self.model
    }

    pub fn rate_limit(&self) -> &RateLimit {
        &self.rate_limit
    }
}
