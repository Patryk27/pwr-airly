#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub country: String,
    pub locality: String,
    pub route: String,
    pub street_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sensor {
    pub id: u32,
    pub name: String,
    pub vendor: String,

    pub location: Location,
    pub address: Address,

    pub pm10: Option<f32>,
    pub pm25: Option<f32>,

    pub air_quality_index: Option<f32>,
    pub pollution_level: Option<i8>,

    pub measurement_time: Option<String>,
}