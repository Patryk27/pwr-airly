#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

pub mod sensor {
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
}

pub mod measurements {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Measurement {
        pub measurement_time: Option<String>,

        pub pm1: Option<f32>,
        pub pm10: Option<f32>,
        pub pm25: Option<f32>,

        pub temperature: Option<f32>,
        pub pressure: Option<f32>,
        pub humidity: Option<f32>,

        pub air_quality_index: Option<f32>,
        pub pollution_level: Option<i8>,

        pub wind_direction: Option<f32>,
        pub wind_speed: Option<f32>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HistoryItem {
        from_date_time: String,
        till_date_time: String,
        measurements: Measurement,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Measurements {
        #[serde(rename = "currentMeasurements")]
        pub current: Measurement,
        pub history: Vec<HistoryItem>,
    }
}