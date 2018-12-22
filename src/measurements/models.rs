use chrono::{DateTime, Utc};

airly_model! {
    Measurements {
        current: Measurement,

        // 24 measurements from the last 24 hours
        history: Vec<Measurement>,

        // 24 measurements forecast for the nest 24 hours
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
        // Measurement's name - e.g. "PM25"
        name: String,

        // Measurement's value - e.g. "21.00"
        value: f32,
    }
}

airly_model! {
    MeasurementIndex {
        // Index's name - e.g. "AIRLY_CAQI"
        name: String,

        // Index's value - its meaning depends on the index, for instance in the "AIRLY_CAQI" index
        // lower value means better air
        value: f32,

        // English string representation of the index's value - e.g. "HIGH"
        level: String,

        // English description based on the index's value - e.g. "Air is quite good."
        description: Option<String>,

        // English advice based on the index's value - e.g. "Take a deep breath. Today, you can. ;)"
        advice: Option<String>,

        // Hex color describing the index's value - e.g. "#D1CF1E"
        color: Option<String>,
    }
}

airly_model! {
    MeasurementStandard {
        // Standard's name - e.g. "WHO"
        name: String,

        // Pollutant to which this value refers to - e.g. "PM25"
        pollutant: String,

        // Standard's limit for the pollutant - e.g. "25.00"
        limit: f32,

        // Ratio between the actually measured value and its limit - e.g. if we've had a 50μg/m3
        // limit and measured 10μg/m3 of PM10 in the air, this field would contain "20.00" (%).
        // Intuitively: value over 100% means that the norm has been exceeded.
        percent: f32,
    }
}
