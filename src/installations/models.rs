airly_model! {
    Installation {
        id: u32,
        elevation: f32,
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
        country: String,
        city: String,
        street: String,
        number: String,
        display_address1: String,
        display_address2: String,
    }
}

airly_model! {
    InstallationSponsor {
        name: String,
        description: String,
        logo: String,
        link: Option<String>,
    }
}
