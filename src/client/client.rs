pub(crate) use reqwest::Client as HttpClient;
pub(crate) use reqwest::Method as HttpMethod;
pub(crate) use reqwest::StatusCode as HttpStatusCode;
use serde::de::DeserializeOwned;
use super::{API_URL, ApiKey, Error, Result};

pub struct Client {
    key: String,
    http: HttpClient,
}

/// This class models a low-level interface facilitating sending requests to the Airly's API.
///
/// # Example
///
/// ```rust
/// let client = Client::new("my-secret-key");
/// let response = client.request(HttpMethod::Get, "installations/100");
///
/// println!("{:#?}", response);
/// ```
impl Client {
    /// Constructs a new instance of the Client.
    ///
    /// The `key` parameter is required and must contain a valid Airly API's key.
    /// You can obtain one by visiting <https://developer.airly.eu>.
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = Client::new("my-secret-key");
    /// ```
    pub fn new(key: String) -> Self {
        Self {
            key,
            http: HttpClient::new(),
        }
    }

    /// Performs a request at given Airly's endpoint and parses the response.
    ///
    /// The URL is automatically prepended with the Airly API's URL (see the example below).
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = client.request(HttpMethod::Get, "installations/100");
    ///
    /// println!("{:#?}", response);
    /// ```
    pub fn request<T: DeserializeOwned>(&self, method: HttpMethod, url: String) -> Result<T> {
        // Start building the request
        let mut request = self.http.request(
            method,
            &format!("{}/{}", API_URL, url),
        );

        // Include the "ApiKey" header
        request.header(
            ApiKey(self.key.clone())
        );

        // Execute the request
        let mut response = request.send()?;

        // Parse the response
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
}
