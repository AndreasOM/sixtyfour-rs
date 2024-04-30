use std::collections::HashMap;

use crate::engine::Uniform;

#[derive(Debug, Default)]
pub struct UniformManager {
    entries: HashMap<String, Uniform>,
}

impl UniformManager {
    pub fn invalidate_locations(&mut self) {
        for (_n, u) in self.entries.iter_mut() {
            u.invalidate_location();
        }
    }
    pub fn add_entry<S>(&mut self, name: S, entry: Uniform) -> Option<Uniform>
    where
        S: Into<String>,
    {
        self.entries.insert(name.into(), entry)
    }

    pub fn entries_mut(&mut self) -> &mut HashMap<String, Uniform> {
        &mut self.entries
    }
    pub fn entries(&self) -> &HashMap<String, Uniform> {
        &self.entries
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Uniform> {
        self.entries.get_mut(name)
    }
}
