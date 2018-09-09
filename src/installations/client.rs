use client::{Client, HttpMethod, Result};
use super::models::*;

pub struct InstallationsClient {
    client: Client,
}

impl InstallationsClient {
    pub fn new(key: String) -> Self {
        Self {
            client: Client::new(key),
        }
    }

    /// Returns an installation with specified id.
    ///
    /// <https://developer.airly.eu/docs#endpoints.installations.getbyid>
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = InstallationsClient::new("my-secret-key");
    /// let installation = client.get(100);
    ///
    /// println!("{:#?}", installation);
    /// ```
    pub fn get(&self, id: u32) -> Result<Installation> {
        self.client.request(
            HttpMethod::Get,
            format!("installations/{}", id),
        )
    }

    /// Returns a list of installations located closest to given point, sorted by distance to that
    /// point.
    ///
    /// <https://developer.airly.eu/docs#endpoints.installations.nearest>
    ///
    /// # Example
    ///
    /// ```rust
    /// let client = InstallationsClient::new("my-secret-key");
    /// let installations = client.get_nearest(50.062006, 19.940984);
    /// ```
    pub fn get_nearest(&self, lat: f32, lng: f32) -> Result<Vec<Installation>> {
        self.client.request(
            HttpMethod::Get,
            format!("installations/nearest?lat={}&lng={}", lat, lng),
        )
    }
}
