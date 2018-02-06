extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use reqwest::Client;
use std::borrow::Borrow;

pub mod models;

pub struct AirlyClient {
    url: String,
    key: String,

    client: Client,
}

type Parameters<'a> = Vec<(&'static str, &'a String)>;

impl AirlyClient {
    /// Creates a new instance of the Airly Client.
    /// In order to obtain your own API key, register at <https://developer.airly.eu>.
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

    /// Returns specific sensor's information (like its address and so on).
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_sensor(984));
    /// ```
    pub fn get_sensor(&self, sensor_id: u32)
        -> Result<models::sensor::Sensor, models::Error>
    {
        self.execute(format!("sensors/{}", sensor_id), vec![])
    }

    /// Returns specific sensor's measurements (like PM10 level and so on).
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_sensor_measurements(984));
    /// ```
    ///
    /// # Errors
    ///
    /// 1. May fail when passed a non-existing sensor's id.
    /// 2. May fail when an invalid API key was specified.
    pub fn get_sensor_measurements(&self, sensor_id: u32)
        -> Result<models::measurements::Measurements, models::Error> {
        self.execute("sensor/measurements".to_string(), vec![
            ("sensorId", &sensor_id.to_string()),
        ])
    }

    /// Returns a sensor located nearest to given location.
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_nearest_sensor(50.0, 19.0).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// 1. May fail when given invalid coordinates or no sensor is found near given location.
    /// 2. May fail when an invalid API key was specified.
    pub fn get_nearest_sensor(&self, latitude: f32, longitude: f32)
        -> Result<models::sensor::Sensor, models::Error> {
        self.execute("nearestSensor/measurements".to_string(), vec![
            ("latitude", &latitude.to_string()),
            ("longitude", &longitude.to_string()),
        ])
    }

    /// Executes a generic request and returns its response.
    /// Provided mainly for internal use, but made public in case someone wants to call own Airly
    /// function.
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = airly.execute("foo/bar", vec![
    ///   ("firstParameter", "firstParameterValue"),
    ///   ("secondParameter", "secondParameterValue"),
    /// ]);
    /// ```
    pub fn execute<T: serde::de::DeserializeOwned>(self: &AirlyClient, action: String, params: Parameters)
        -> Result<T, models::Error> {
        use reqwest::Url;

        // prepare the request URL
        let mut request_url = Url::parse(format!("{}/{}", self.url, action).borrow()).unwrap();

        // prepare the request parameters
        {
            let mut request_params = request_url.query_pairs_mut();

            for param in params {
                request_params.append_pair(param.0, param.1);
            }

            request_params.append_pair("apikey", &self.key);
        }

        // execute the request
        let mut response = self.client.get(request_url).send().unwrap();

        match response.status() {
            // -- if received 200 OK, unserialize like a regular message -- //
            reqwest::StatusCode::Ok => {
                let result = response.json::<T>();

                match result {
                    Ok(result) => Ok(result),

                    // it may happen that we receive a valid response, but nonetheless fail to parse
                    // it - return a meaningful error message in such case
                    Err(msg) => Err(
                        models::Error {
                            message: format!("Request succeeded, but we failed to parse the response: {}", msg),
                        }
                    )
                }
            }

            // -- if received any other code, unserialize like an error -- //
            _ => {
                let result = response.json::<models::Error>();

                match result {
                    Ok(result) => Err(result),

                    // it may happen that we receive a completely invalid error message, which is
                    // not a JSON - return a meaningful error message in such case
                    Err(msg) => Err(
                        models::Error {
                            message: format!("Request failed and additionally we failed to parse the response: {}", msg),
                        }
                    )
                }
            }
        }
    }
}