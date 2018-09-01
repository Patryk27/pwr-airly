use client::Client;
use super::models::*;

pub struct InstallationsClient {
    client: Client,
}

impl InstallationsClient {

    pub fn new(client: Client) -> InstallationsClient {
        InstallationsClient {
            client,
        }
    }

    pub fn get(&self, id: u32) -> Installation {
        self.client.get(
            format!("installations/{}", id)
        )
    }

}