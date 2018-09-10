use reqwest::Client as HttpClient;
use reqwest::Method as HttpMethod;
use reqwest::RequestBuilder as HttpRequestBuilder;

header! { (ApiKey, "ApiKey") => [String] }

pub struct RequestBuilder {
    method: Option<HttpMethod>,
    url: Option<String>,
    key: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            method: None,
            url: None,
            key: None,
        }
    }

    pub fn set_method(&mut self, method: HttpMethod) {
        self.method = Some(method);
    }

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn set_key(&mut self, key: String) {
        self.key = Some(key);
    }

    pub fn build(self, http: &HttpClient) -> HttpRequestBuilder {
        let mut request = http.request(
            self.method.unwrap(),
            &format!("https://airapi.airly.eu/v2/{}", self.url.unwrap()),
        );

        request.header(
            ApiKey(
                self.key.unwrap()
            )
        );

        request
    }
}
