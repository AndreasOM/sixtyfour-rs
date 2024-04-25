use crate::property_manager::PropertyManager;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub property_manager: PropertyManager,
}

impl State {}
