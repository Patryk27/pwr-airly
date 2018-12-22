/// An unofficial client for the [Airly's v2 API](http://developer.airly.eu).
///
/// # License
///
/// Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.
#[macro_use]
extern crate serde_derive;

use self::installations::InstallationsClient;
use self::measurements::MeasurementsClient;

#[macro_use]
mod models;

mod client;
mod installations;
mod measurements;

pub struct AirlyClient {
    installations: InstallationsClient,
    measurements: MeasurementsClient,
}

impl AirlyClient {
    /// Creates a new instance of the `AirlyClient`.
    ///
    /// To connect to the Airly's API you'll need your own API key, which you can obtain by
    /// registering at <https://developer.airly.eu> (it's free!).
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
