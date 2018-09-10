use chrono::{DateTime, Utc};

airly_model! {
    Measurements {
        current: Measurement,
        history: Vec<Measurement>,
        forecast: Vec<Measurement>,
    }
}

airly_model! {
    Measurement {
        #[serde(rename = "from_date_time")]
        from: Option<DateTime<Utc>>,

        #[serde(rename = "till_date_time")]
        till: Option<DateTime<Utc>>,

        values: Vec<MeasurementValue>,
        indexes: Vec<MeasurementIndex>,
        standards: Vec<MeasurementStandard>,
    }
}

airly_model! {
    MeasurementValue {
        name: String,
        value: f32,
    }
}

airly_model! {
    MeasurementIndex {
        name: String,
        value: Option<f32>,
        level: Option<String>,
        description: Option<String>,
        advice: Option<String>,
        color: Option<String>,
    }
}

airly_model! {
    MeasurementStandard {
        name: String,
        pollutant: String,
        limit: f32,
        percent: f32,
    }
}
