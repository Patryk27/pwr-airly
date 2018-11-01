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

        impl ::models::Model for $name {
            // Nottin' here - it functions as a marker only
        }
    )
}

pub trait Model: Clone + DeserializeOwned {
    // Nottin' here - it functions as a marker only
}

// A vector of models is still considered a model.
//
// It allows to neatly organize all the actions that return a vector of models instead of a single
// model (e.g. "get nearest installations").
impl<T: Model> Model for Vec<T> {
    // Nottin' here - it functions as a marker only
}
