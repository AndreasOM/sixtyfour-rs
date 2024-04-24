use crate::engine::UniformManager;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct PropertyManager {
    entries: HashMap<String, f32>,
}

impl PropertyManager {
    pub fn add_entry<S>(&mut self, name: S, entry: f32) -> Option<f32>
    where
        S: Into<String>,
    {
        self.entries.insert(name.into(), entry)
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, f32> {
        &mut self.entries
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut f32> {
        self.entries.get_mut(name)
    }
    pub fn ensure_property_f32(&mut self, name: &str, default_value: f32) {
        if !self.entries.contains_key(name) {
            self.entries.insert(name.into(), default_value);
        }
    }

    pub fn ensure_all_properties_from_uniforms(&mut self, uniform_manager: &UniformManager) {
        for (k, _v) in uniform_manager.entries().iter() {
            self.ensure_property_f32(k, 1.0);
        }
    }
}
