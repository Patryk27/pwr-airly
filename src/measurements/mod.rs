use reqwest::Method as HttpMethod;

use crate::client::Client;
use crate::client::Response;
use crate::client::Result;

pub use self::models::*;

mod models;

/// This structure models all the actions related to the `/v2/measurements` namespace.
/// You shouldn't use it on your own - refer to the `AirlyClient` facade instead.
pub struct MeasurementsClient {
    client: Client,
}

impl MeasurementsClient {
    pub fn new(key: String) -> Self {
        Self {
            client: Client::new(key),
        }
    }

    /// Returns measurements for installation with specified id.
    /// <https://developer.airly.eu/docs#endpoints.measurements.installation>
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = AirlyClient::new("my-api-key");
    /// println!("{:#?}", client.measurements().for_installation(250));
    /// ```
    pub fn get(&self, installation_id: u32) -> Result<Response<Measurements>> {
        self.client.perform(
            HttpMethod::GET,
            format!("measurements/installation?installationId={}", installation_id),
        )
    }

    /// Returns measurements for specific point.
    /// <https://developer.airly.eu/docs#endpoints.measurements.point>
    pub fn for_point(&self, lat: f32, lng: f32) -> Result<Response<Measurements>> {
        self.client.perform(
            HttpMethod::GET,
            format!("measurements/point?lat={}&lng={}", lat, lng),
        )
    }
}
