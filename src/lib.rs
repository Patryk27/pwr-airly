/// An unofficial client for the [Airly](http://developer.airly.eu) v2 API.
///
/// # License
///
/// Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.

extern crate chrono;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use installations::InstallationsClient;
use measurements::MeasurementsClient;

#[macro_use]
mod models;

mod installations;
mod measurements;
mod utils;

pub struct AirlyClient {
    installations: InstallationsClient,
    measurements: MeasurementsClient,
}

impl AirlyClient {
    /// Creates a new instance of the `AirlyClient`.
    ///
    /// To create an instance you'll need your own API key, which you can obtain by registering at
    /// <https://developer.airly.eu> (it's free!).
    ///
    /// # Example
    ///
    /// ```rust
    /// let airly = AirlyClient::new("my-api-key");
    /// ```
    pub fn new<K: Into<String>>(key: K) -> AirlyClient {
        let key = key.into();

        AirlyClient {
            installations: InstallationsClient::new(key.clone()),
            measurements: MeasurementsClient::new(key.clone()),
        }
    }

    pub fn installations(&self) -> &InstallationsClient {
        &self.installations
    }

    pub fn measurements(&self) -> &MeasurementsClient {
        &self.measurements
    }
}
