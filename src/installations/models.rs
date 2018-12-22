airly_model! {
    Installation {
        id: u32,

        /// Attitude at which this device is installed, represented in MAMSL
        elevation: f32,

        /// A flag indicating whether this installation is an Airly device or not
        airly: bool,

        location: InstallationLocation,
        address: InstallationAddress,
        sponsor: InstallationSponsor,
    }
}

airly_model! {
    InstallationLocation {
        latitude: f32,
        longitude: f32,
    }
}

airly_model! {
    InstallationAddress {
        country: Option<String>,
        city: Option<String>,
        street: Option<String>,
        number: Option<String>,
        display_address1: Option<String>,
        display_address2: Option<String>,
    }
}

airly_model! {
    InstallationSponsor {
        name: String,
        description: Option<String>,

        /// URL to the sponsor's logo
        logo: Option<String>,

        /// URL to the sponsor's site
        link: Option<String>,
    }
}
