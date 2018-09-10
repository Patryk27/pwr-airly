use client::{Client, HttpMethod, Response, Result};
use measurements::models::*;

pub struct MeasurementsClient {
    client: Client,
}

impl MeasurementsClient {
    pub fn new(key: String) -> Self {
        Self {
            client: Client::new(key),
        }
    }

    pub fn get(&self, installation_id: u32) -> Result<Response<Measurements>> {
        self.client.request(
            HttpMethod::Get,
            format!("measurements/installation?installationId={}", installation_id),
        )
    }
}
