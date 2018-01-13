extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod models;

use reqwest::{Client, Response};
use std::borrow::Borrow;

pub struct AirlyClient {
    url: String,
    key: String,

    client: Client,
}

type Parameters<'a> = Vec<(&'static str, &'a String)>;

impl AirlyClient {
    /// Create a new instance of the Airly Client.
    /// In order to obtain your own API key, register at https://developer.airly.eu
    ///
    /// # Example
    ///
    /// ```rust
    /// let airly = AirlyClient::new("my-secret-key");
    /// ```
    pub fn new(key: &'static str) -> AirlyClient {
        AirlyClient {
            url: "http://airapi.airly.eu/v1".into(),
            key: String::from(key),

            client: Client::new(),
        }
    }

    /// Get a sensor located nearest to given location.
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_nearest_sensor(50.0, 19.0));
    /// ```
    ///
    /// # Errors
    ///
    /// May fail when given invalid coordinates or no sensor is found near given location.
    pub fn get_nearest_sensor(self: &AirlyClient, latitude: f32, longitude: f32) -> models::sensor::Sensor {
        self.execute("nearestSensor/measurements", vec![
            ("latitude", &latitude.to_string()),
            ("longitude", &longitude.to_string()),
        ]).json().unwrap()
    }

    /// Get measurements for a specific sensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// let sensor = airly.get_nearest_sensor(50.0, 19.0);
    /// println!("{:#?}", airly.get_sensor_measurements(sensor.id));
    /// ```
    ///
    /// # Errors
    ///
    /// May fail when passed a non-existing sensor's id.
    pub fn get_sensor_measurements(self: &AirlyClient, sensor_id: u32) -> models::measurements::Measurements {
        self.execute("sensor/measurements", vec![
            ("sensorId", &sensor_id.to_string()),
        ]).json().unwrap()
    }

    /// Execute request and return response.
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = airly.execute("foo/bar", vec![
    ///   ("firstParameter", "firstParameterValue"),
    ///   ("secondParameter", "secondParameterValue"),
    /// ]);
    /// ```
    pub fn execute(self: &AirlyClient, action: &'static str, params: Parameters) -> Response {
        use reqwest::Url;

        // prepare the request URL
        let mut request_url = Url::parse((self.url.clone() + "/" + action).borrow()).unwrap();

        // prepare the request parameters
        {
            let mut request_params = request_url.query_pairs_mut();

            for param in params {
                request_params.append_pair(param.0, param.1);
            }

            request_params.append_pair("apikey", &self.key);
        }

        // execute the request
        self.client.get(request_url).send().unwrap()
    }
}