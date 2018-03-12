/// A simple wrapper for the [Airly](http://developer.airly.eu) API, for version 1.
/// Build upon [reqwest](https://github.com/seanmonstar/reqwest) & [serde}(https://github.com/serde-rs/serde).
///
/// # License
///
/// Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use reqwest::Client;
pub use reqwest::Error as ReqwestError;
use reqwest::Response;
use reqwest::Url;
pub use reqwest::UrlError as ReqwestUrlError;

pub mod error;
mod internal;
pub mod models;

type Parameter<'a> = (&'static str, &'a String);
type Parameters<'a> = Vec<Parameter<'a>>;

const URL: &str = "http://airapi.airly.eu/v1";

pub type Result<T> = ::std::result::Result<T, error::Error>;

pub struct AirlyClient {
    key: String,
    client: Client,
}

impl AirlyClient {
    /// Creates a new instance of the Airly Client.
    /// In order to obtain your own API key, register at <https://developer.airly.eu>.
    ///
    /// # Example
    ///
    /// ```rust
    /// let airly = AirlyClient::new("my-secret-key");
    /// ```
    pub fn new<T: Into<String>>(key: T) -> AirlyClient {
        AirlyClient {
            key: key.into(),
            client: Client::new(),
        }
    }

    /// Returns specific sensor's information (like its address and so on).
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_sensor(984).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// 1. May fail when given invalid sensor id.
    /// 2. May fail when an invalid API key was specified.
    pub fn get_sensor(&self, sensor_id: u32)
        -> Result<models::sensor::Sensor>
    {
        self.execute(format!("sensors/{}", sensor_id), vec![])
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
        -> Result<models::sensor::Sensor> {
        self.execute("nearestSensor/measurements".to_string(), vec![
            ("latitude", &latitude.to_string()),
            ("longitude", &longitude.to_string()),
        ])
    }

    /// Returns measurements for a specified sensor.
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_sensor_measurements(984).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// 1. May fail when passed a non-existing sensor's id.
    /// 2. May fail when an invalid API key was specified.
    pub fn get_sensor_measurements(&self, sensor_id: u32)
        -> Result<models::measurements::Measurements> {
        self.execute("sensor/measurements".to_string(), vec![
            ("sensorId", &sensor_id.to_string()),
        ])
    }

    /// Returns measurements for a specified point on the map.
    ///
    /// # Example
    ///
    /// ```rust
    /// println!("{:#?}", airly.get_map_point_measurements(50.06, 19.93).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// 1. May fail when passed an invalid coordinates.
    /// 2. May fail when an invalid API key was specified.
    pub fn get_map_point_measurements(&self, latitude: f32, longitude: f32)
        -> Result<models::measurements::Measurements> {
        self.execute("mapPoint/measurements".to_string(), vec![
            ("latitude", &latitude.to_string()),
            ("longitude", &longitude.to_string()),
        ])
    }

    /// Executes a generic request and returns its response.
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = airly.execute("foo/bar", vec![
    ///   ("firstParameter", "firstParameterValue"),
    ///   ("secondParameter", "secondParameterValue"),
    /// ]);
    /// ```
    fn execute<T>(self: &AirlyClient, action: String, parameters: Parameters) -> Result<T>
        where T: serde::de::DeserializeOwned {
        // build a request
        let request = self.build_request(action, parameters)?;

        // execute it
        let mut response = self.client.get(request).send()?;

        // and, eventually, parse the response
        AirlyClient::parse_response(&mut response)
    }
}