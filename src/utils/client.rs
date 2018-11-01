use models::Model;
use reqwest::Client as HttpClient;
use reqwest::Method as HttpMethod;
use utils::Response;
use utils::ResponseParser;
use utils::Result;

pub struct Client {
    key: String,
    http: HttpClient,
}

impl Client {
    pub fn new(key: String) -> Self {
        Self {
            key,
            http: HttpClient::new(),
        }
    }

    pub fn perform<T: Model>(&self, method: HttpMethod, url: String) -> Result<Response<T>> {
        // Build the request
        let request = self.http.request(
            method,
            &format!("https://airapi.airly.eu/v2/{}", url),
        );

        let request = request.header("ApiKey", self.key.clone());

        // Execute it & parse the response
        ResponseParser::parse(&mut request.send()?)
    }
}
