use models::Model;
use reqwest::Response as HttpResponse;
use reqwest::StatusCode as HttpStatusCode;
use utils::Error;
use utils::RateLimit;
use utils::Response;
use utils::Result;

pub struct ResponseParser;

impl ResponseParser {
    pub fn parse<T: Model>(response: &mut HttpResponse) -> Result<Response<T>> {
        Ok(Response::new(
            Self::parse_model(response)?,
            Self::parse_rate_limit(response),
        ))
    }

    fn parse_model<T: Model>(response: &mut HttpResponse) -> Result<T> {
        match response.status() {
            // HTTP 200 - Ok
            HttpStatusCode::OK => response
                .json()
                .map_err(|error| Error::InvalidResponse(Box::new(error))),

            // HTTP 401 - Unauthorized / HTTP 403 - Forbidden
            HttpStatusCode::UNAUTHORIZED | HttpStatusCode::FORBIDDEN => Err(
                Error::InvalidCredentials(
                    response.json()?,
                ),
            ),

            // HTTP 404 - Not Found
            HttpStatusCode::NOT_FOUND => Err(
                Error::ResourceNotFound(
                    response.json()?
                ),
            ),

            // HTTP 429 - Too Many Requests
            HttpStatusCode::TOO_MANY_REQUESTS => Err(
                Error::RateLimitReached(
                    response.json()?
                ),
            ),

            _ => Err(Error::UnexpectedHttpStatusCode(response.status().as_u16()))
        }
    }

    fn parse_rate_limit(response: &mut HttpResponse) -> RateLimit {
        let headers = response.headers();

        let parse_header = move |name| {
            headers
                .get(name)
                .map(|value| value.to_str().unwrap())
                .map(|value| value.parse().unwrap())
                .unwrap_or(0)
        };

        RateLimit {
            daily_limit: parse_header("X-RateLimit-Limit-Day"),
            daily_remaining: parse_header("X-RateLimit-Remaining-Day"),
            minute_limit: parse_header("X-RateLimit-Limit-Minute"),
            minute_remaining: parse_header("X-RateLimit-Remaining-Minute"),
        }
    }
}
