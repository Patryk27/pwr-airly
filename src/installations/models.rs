airly_model! {
    pub struct Installation {
        pub id: u32,
        pub elevation: f32,
        pub airly: bool,

        pub location: InstallationLocation,
        pub address: InstallationAddress,
        pub sponsor: InstallationSponsor,
    }
}

airly_model! {
    pub struct InstallationLocation {
        pub latitude: f32,
        pub longitude: f32,
    }
}

airly_model! {
    pub struct InstallationAddress {
        pub country: String,
        pub city: String,
        pub street: String,
        pub number: String,
        pub display_address1: String,
        pub display_address2: String,
    }
}

airly_model! {
    pub struct InstallationSponsor {
        pub name: String,
        pub description: String,
        pub logo: String,
        pub link: Option<String>,
    }
}