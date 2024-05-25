use std::collections::HashMap;

use crate::project::ResourceId;

#[derive(Debug, Default)]
pub struct ResourceLogManager {
    logs: HashMap<ResourceId, Vec<String>>,
}

impl ResourceLogManager {
    pub fn add(&mut self, resource_id: ResourceId, log: Vec<String>) {
        self.logs.insert(resource_id, log);
    }

    pub fn get(&self, resource_id: &ResourceId) -> Option<&Vec<String>> {
        self.logs.get(resource_id)
    }
}
