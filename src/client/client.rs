use client::{request_builder::RequestBuilder, Response, response_parser::ResponseParser, Result};
use models::Model;
use reqwest::Client as HttpClient;
use reqwest::Method as HttpMethod;

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
    pub fn request<T: Model>(&self, method: HttpMethod, url: String) -> Result<Response<T>> {
        // Build the request
        let mut request_builder = RequestBuilder::new();

        request_builder.set_method(method);
        request_builder.set_url(url);
        request_builder.set_key(self.key.clone());

        let mut request = request_builder.build(&self.http);

        // Execute it & parse the response
        ResponseParser::parse(
            &mut request.send()?
        )
    }
}
