use reqwest::Client as HttpClient;
use reqwest::Method as HttpMethod;

use crate::client::parse_response;
use crate::client::Response;
use crate::client::Result;
use crate::models::Model;

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
        let url = format!("https://airapi.airly.eu/v2/{}", url);

        let request = self.http
            .request(method, &url)
            .header("ApiKey", self.key.clone());

        parse_response(&mut request.send()?)
    }
}
