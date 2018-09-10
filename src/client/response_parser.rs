use client::{Error, RateLimit, Response, Result};
use models::Model;
use reqwest::Response as HttpResponse;
use reqwest::StatusCode as HttpStatusCode;

header! { (XRateLimitLimitDay, "X-RateLimit-Limit-Day") => [u32] }
header! { (XRateLimitRemainingDay, "X-RateLimit-Remaining-Day") => [u32] }
header! { (XRateLimitLimitMinute, "X-RateLimit-Limit-Minute") => [u32] }
header! { (XRateLimitRemainingMinute, "X-RateLimit-Remaining-Minute") => [u32] }

pub struct ResponseParser {
    // Nottin' here
}

impl ResponseParser {
    pub fn parse<T: Model>(response: &mut HttpResponse) -> Result<Response<T>> {
        Ok(
            Response::new(
                Self::parse_model(response)?,
                Self::parse_rate_limit(response),
            )
        )
    }

    fn parse_model<T: Model>(response: &mut HttpResponse) -> Result<T> {
        match response.status() {
            // HTTP 200 - Ok
            HttpStatusCode::Ok => response.json()
                .map_err(|error| {
                    Error::Unknown(
                        format!("Failed to parse the response's JSON - this shouldn't happen. Error message: {}", error)
                    )
                }),

            // HTTP 401 - Unauthorized or HTTP 403 - Forbidden
            HttpStatusCode::Unauthorized | HttpStatusCode::Forbidden => Err(
                Error::InvalidKey(
                    response.json()?,
                ),
            ),

            // HTTP 404 - Not Found
            HttpStatusCode::NotFound => Err(
                Error::ResourceNotFound(
                    response.json()?
                ),
            ),

            // HTTP 429 - Too Many Requests
            HttpStatusCode::TooManyRequests => Err(
                Error::TooManyRequests(
                    response.json()?
                ),
            ),

            // If API's returned other status code, bail out
            _ => Err(
                Error::Unknown(
                    format!("API has returned an unrecognized HTTP status code: [{}].", response.status())
                )
            )
        }
    }

    fn parse_rate_limit(response: &mut HttpResponse) -> RateLimit {
        let headers = response.headers();

        RateLimit {
            daily_limit: headers.get::<XRateLimitLimitDay>().map(|value| **value).unwrap_or(0),
            daily_remaining: headers.get::<XRateLimitRemainingDay>().map(|value| **value).unwrap_or(0),

            minute_limit: headers.get::<XRateLimitLimitMinute>().map(|value| **value).unwrap_or(0),
            minute_remaining: headers.get::<XRateLimitRemainingMinute>().map(|value| **value).unwrap_or(0),
        }
    }
}
