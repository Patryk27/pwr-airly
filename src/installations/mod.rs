use reqwest::Method as HttpMethod;
pub use self::models::*;
use utils::Client;
use utils::Response;
use utils::Result;

mod models;

/// This structure models all the actions related to the `/v2/installations` namespace.
/// You shouldn't use it on your own - refer to the `AirlyClient` facade instead.
pub struct InstallationsClient {
    client: Client,
}

impl InstallationsClient {
    pub fn new(key: String) -> Self {
        Self {
            client: Client::new(key),
        }
    }

    /// Returns installation with specified id.
    /// <https://developer.airly.eu/docs#endpoints.installations.getbyid>
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = AirlyClient::new("my-api-key");
    /// println!("{:#?}", client.installations().get(250));
    /// ```
    pub fn get(&self, id: u32) -> Result<Response<Installation>> {
        self.client.perform(
            HttpMethod::GET,
            format!("installations/{}", id),
        )
    }

    /// Returns installations located closest to given point, sorted by distance to it.
    /// <https://developer.airly.eu/docs#endpoints.installations.nearest>
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = AirlyClient::new("my-api-key");
    /// println!("{:#?}", client.installations().get_nearest(50.062006, 19.940984));
    /// ```
    pub fn get_nearest(&self, lat: f32, lng: f32) -> Result<Response<Vec<Installation>>> {
        self.client.perform(
            HttpMethod::GET,
            format!("installations/nearest?lat={}&lng={}", lat, lng),
        )
    }
}
