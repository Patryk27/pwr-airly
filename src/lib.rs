/// A simple client for the [Airly](http://developer.airly.eu) v2 API.
///
/// # License
///
/// Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.

#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use client::Client;
use installations::InstallationsClient;

mod installations;
mod client;

pub struct AirlyClient {
    installations_client: InstallationsClient,
}

/**
 * airly.installations().get(id)               -> Installation
 * airly.installations().get_nearest(lat, lng) -> Vec<Installation>
 *
 * airly.measurements().get_by_installation(id)               -> Measurements
 * airly.measurements().get_by_nearest_installation(lat, lng) -> Measurements
 * airly.measurements().get_by_point(lat, lng)                -> Measurements
 *
 * airly.meta().get_indexes()      -> Vec<IndexMeta>
 * airly.meta().get_measurements() -> Vec<MeasurementMeta>
 */
impl AirlyClient {
    /// Creates a new instance of the Airly Client.
    /// In order to obtain your own API key, register at <https://developer.airly.eu>.
    ///
    /// # Example
    ///
    /// ```rust
    /// let airly = AirlyClient::new("my-secret-key");
    /// ```
    pub fn new<K: Into<String>>(key: K) -> AirlyClient {
        let client = Client::new(
            key.into()
        );

        AirlyClient {
            installations_client: InstallationsClient::new(client),
        }
    }

    pub fn installations(&self) -> &InstallationsClient {
        &self.installations_client
    }
}