use crate::engine::UniformManager;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct Property {
    pub value: PropertyValue,
    pub config: PropertyConfig,
}

impl Property {
    pub fn default_f32(value: f32) -> Self {
        Self {
            value: PropertyValue::F32 { value },
            config: PropertyConfig::default_f32(),
        }
    }
    pub fn value(&self) -> &PropertyValue {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut PropertyValue {
        &mut self.value
    }

    pub fn config(&self) -> &PropertyConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut PropertyConfig {
        &mut self.config
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub enum PropertyValue {
    F32 {
        value: f32,
    },
    Bool {
        value: bool,
    },
    #[default]
    None,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub enum PropertyConfig {
    F32 {
        min_value: f32,
        max_value: f32,
        step_size: f32,
    },
    Bool {},
    #[default]
    None,
}

impl PropertyConfig {
    pub fn default_f32() -> Self {
        Self::F32 {
            min_value: 0.0,
            max_value: 100.0,
            step_size: 1.0,
        }
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct PropertyManager {
    entries: HashMap<String, Property>,
}

impl PropertyManager {
    pub fn add_entry<S>(&mut self, name: S, entry: Property) -> Option<Property>
    where
        S: Into<String>,
    {
        self.entries.insert(name.into(), entry)
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, Property> {
        &mut self.entries
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Property> {
        self.entries.get_mut(name)
    }
    pub fn ensure_property_f32(&mut self, name: &str, default_value: f32) {
        if !self.entries.contains_key(name) {
            self.entries
                .insert(name.into(), Property::default_f32(default_value));
        } else {
            // :TODO: ensure type is correct
        }
    }

    pub fn ensure_all_properties_from_uniforms(&mut self, uniform_manager: &UniformManager) {
        for (k, _v) in uniform_manager.entries().iter() {
            // :TODO: handle uniform type
            self.ensure_property_f32(k, 1.0);
        }
    }
}
