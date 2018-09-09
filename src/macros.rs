macro_rules! airly_model {
    ($struct_def:item) => (
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        $struct_def
    )
}