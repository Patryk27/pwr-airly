use reqwest::Client as HttpClient;
use hyper::header::Headers;
use reqwest::Url;
use serde::de::DeserializeOwned;

header! { (ApiKey, "ApiKey") => [String] }

const API_URL: &str = "http://airapi.airly.eu/v2/";

pub struct Client {
    key: String,
    http: HttpClient,
}

impl Client {
    pub fn new(key: String) -> Client {
        Client {
            key,
            http: HttpClient::new(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, url: String) -> T {
        let mut request = self.http.get(
            Url::parse(
                &format!("{}/{}", API_URL, url)
            ).unwrap()
        );

        request.header(
            ApiKey::new(self.key)
        );

        let mut response = request.send().unwrap();

        println!("{:?}", response.text());

        response.json().unwrap()
    }
}
