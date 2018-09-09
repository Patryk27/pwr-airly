use chrono::{DateTime, Utc};

airly_model! {
    pub struct Measurements {
        pub current: Measurement,
        pub history: Vec<Measurement>,
        pub forecast: Vec<Measurement>,
    }
}

airly_model! {
    pub struct Measurement {
        #[serde(rename = "from_date_time")]
        pub from: Option<DateTime<Utc>>,

        #[serde(rename = "till_date_time")]
        pub till: Option<DateTime<Utc>>,

        pub values: Vec<MeasurementValue>,
        pub indexes: Vec<MeasurementIndex>,
        pub standards: Vec<MeasurementStandard>,
    }
}

airly_model! {
    pub struct MeasurementValue {
        pub name: String,
        pub value: f32,
    }
}

airly_model! {
    pub struct MeasurementIndex {
        pub name: String,
        pub value: Option<f32>,
        pub level: Option<String>,
        pub description: Option<String>,
        pub advice: Option<String>,
        pub color: Option<String>,
    }
}

airly_model! {
    pub struct MeasurementStandard {
        pub name: String,
        pub pollutant: String,
        pub limit: f32,
        pub percent: f32,
    }
}
