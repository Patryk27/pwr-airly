use serde::de::DeserializeOwned;

macro_rules! airly_model {
    (
        $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_name:ident: $field_type:ty,
            )*
        }
    ) => (
        #[derive(Clone, Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field_name: $field_type,
            )*
        }

        impl crate::models::Model for $name {
            // Nottin' here - it's a marker-trait
        }
    )
}

pub trait Model: Clone + DeserializeOwned {
    // Nottin' here - it's a marker-trait
}

/// A vector of models is still considered a model - it allows us to neatly organize all the actions
/// that return a vector of models.
impl<T: Model> Model for Vec<T> {}
