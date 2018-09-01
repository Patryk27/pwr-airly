#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Installation {
    pub id: u32,
    pub elevation: f32,
    pub airly: bool,

    pub location: InstallationLocation,
    pub address: InstallationAddress,
    pub sponsor: InstallationSponsor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationLocation {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationAddress {
    pub country: String,
    pub city: String,
    pub street: String,
    pub number: String,
    pub display_address1: String,
    pub display_address2: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationSponsor {
    pub name: String,
    pub description: String,
    pub logo: String,
    pub link: String,
}