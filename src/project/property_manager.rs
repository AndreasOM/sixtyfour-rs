use crate::engine::UniformManager;
use crate::engine::UniformType;
use std::collections::BTreeMap;
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
    pub fn default_vec2_f32(values: &[f32; 2]) -> Self {
        Self {
            value: PropertyValue::Vec2F32 { values: *values },
            config: PropertyConfig::default_f32(),
        }
    }
    pub fn default_vec3_f32(values: &[f32; 3]) -> Self {
        Self {
            value: PropertyValue::Vec3F32 { values: *values },
            config: PropertyConfig::default_f32(),
        }
    }
    pub fn default_vec3_f32_size4(values: &[f32; 3 * 4]) -> Self {
        Self {
            value: PropertyValue::Vec3F32Size4 { values: *values },
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
    Vec2F32 {
        values: [f32; 2],
    },
    Vec3F32 {
        values: [f32; 3],
    },
    Vec3F32Size4 {
        // :HACK:
        values: [f32; 3 * 4],
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
    ColorRgb {},
    ColorPal {},
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
    //    entries: HashMap<String, Property>,
    entries: BTreeMap<String, Property>,

    #[serde(skip)]
    version: u32,
}

impl PropertyManager {
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn add_entry<S>(&mut self, name: S, entry: Property) -> Option<Property>
    where
        S: Into<String>,
    {
        self.version += 1;
        self.entries.insert(name.into(), entry)
    }

    pub fn delete_entry(&mut self, name: &str) -> Option<Property> {
        self.version += 1;
        self.entries.remove(name)
    }

    pub fn entries_mut(&mut self) -> &mut BTreeMap<String, Property> {
        &mut self.entries
    }

    pub fn entries(&self) -> &BTreeMap<String, Property> {
        &self.entries
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Property> {
        self.entries.get_mut(name)
    }

    pub fn wipe_all(&mut self) {
        self.version += 1;
        self.entries.clear();
    }
    pub fn ensure_property_f32(&mut self, name: &str, default_value: f32) {
        if !self.entries.contains_key(name) {
            self.add_entry(name, Property::default_f32(default_value));
            /*
            self.entries
                .insert(name.into(), Property::default_f32(default_value));
                */
        } else {
            // :TODO: ensure type is correct
        }
    }

    pub fn ensure_property_vec2_f32(&mut self, name: &str, default_values: &[f32; 2]) {
        if !self.entries.contains_key(name) {
            self.add_entry(name, Property::default_vec2_f32(default_values));
            /*
            self.entries
                .insert(name.into(), Property::default_vec2_f32(default_values));
            */
        } else {
            // :TODO: ensure type is correct
        }

        /* :TODO:
                if let Some(p) = self.entries.get_mut(name) {
                    if name.ends_with("_rgb") {
                        match p.config {
                            PropertyConfig::ColorRgb {} => {}
                            _ => {
                                p.config = PropertyConfig::ColorRgb {};
                            }
                        }
                    }
                }
        */
    }

    pub fn ensure_property_vec3_f32(&mut self, name: &str, default_values: &[f32; 3]) {
        if !self.entries.contains_key(name) {
            self.add_entry(name, Property::default_vec3_f32(default_values));
            /*
            self.entries
                .insert(name.into(), Property::default_vec3_f32(default_values));
            */
        } else {
            // :TODO: ensure type is correct
        }

        if let Some(p) = self.entries.get_mut(name) {
            if name.ends_with("_rgb") {
                match p.config {
                    PropertyConfig::ColorRgb {} => {}
                    _ => {
                        p.config = PropertyConfig::ColorRgb {};
                    }
                }
            }
        }
    }
    pub fn ensure_property_vec3_f32_size4(&mut self, name: &str, default_values: &[f32; 3 * 4]) {
        if !self.entries.contains_key(name) {
            self.add_entry(name, Property::default_vec3_f32_size4(default_values));
            /*
            self.entries.insert(
                name.into(),
                Property::default_vec3_f32_size4(default_values),
            );
            */
        } else {
            // :TODO: ensure type is correct
        }

        if let Some(p) = self.entries.get_mut(name) {
            let mut parts = name.split("[");
            let n = parts.next().unwrap(); // safe since split always returns at least one element

            if n.ends_with("_pal") {
                match p.config {
                    PropertyConfig::ColorPal {} => {}
                    _ => {
                        p.config = PropertyConfig::ColorPal {};
                    }
                }
            }
        }
    }

    pub fn ensure_all_properties_from_uniforms(&mut self, uniform_manager: &UniformManager) {
        for (k, v) in uniform_manager.entries().iter() {
            match v.ttype() {
                UniformType::Float => self.ensure_property_f32(k, 1.0),
                UniformType::Vec2Float => self.ensure_property_vec2_f32(k, &[1.0, 1.0]),
                UniformType::Vec3Float => match v.size() {
                    1 => self.ensure_property_vec3_f32(k, &[1.0, 1.0, 1.0]),
                    4 => self.ensure_property_vec3_f32_size4(k, &[1.0; 3 * 4]),
                    o => eprintln!("Size {o} not supported for Vec3Float -> {k}"),
                },
                o => {
                    eprintln!("No matching property for {o:?}");
                }
            }
        }
    }
}
