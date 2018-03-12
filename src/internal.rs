use std::borrow::Borrow;
use super::*;

impl AirlyClient {
    /// Builds a request (URL) from given parameters.
    /// Used in the `execute()` method.
    pub fn build_request(self: &AirlyClient, action: String, parameters: Parameters) -> Result<Url> {
        let mut request = Url::parse(format!("{}/{}", URL, action).borrow())?;

        {
            let mut request_parameters = request.query_pairs_mut();

            for param in parameters {
                request_parameters.append_pair(param.0, param.1);
            }

            request_parameters.append_pair("apikey", &self.key);
        }

        Ok(request)
    }

    /// Parses the request's response.
    /// Used in the `execute()` method.
    pub fn parse_response<T>(response: &mut Response) -> Result<T>
        where T: serde::de::DeserializeOwned {
        match response.status() {
            // when we've received "200 OK", un-json the response like a regular, correct one
            reqwest::StatusCode::Ok => Ok(response.json::<T>()?),

            // when we've received any other code, un-json the response like an error
            _ => Err(
                error::Error::ApiError(
                    response.json::<models::error::Error>()?
                )
            )
        }
    }
}