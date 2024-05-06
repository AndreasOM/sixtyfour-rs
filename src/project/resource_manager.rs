use crate::project::Resource;
use crate::project::ResourceId;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct ResourceManager {
    resources: HashMap<ResourceId, Resource>,
}

impl ResourceManager {
    pub fn resources(&self) -> &HashMap<ResourceId, Resource> {
        &self.resources
    }
    pub fn resources_mut(&mut self) -> &mut HashMap<ResourceId, Resource> {
        &mut self.resources
    }

    pub fn get(&self, resource_id: &ResourceId) -> Option<&Resource> {
        self.resources.get(resource_id)
    }
    pub fn get_mut(&mut self, resource_id: &ResourceId) -> Option<&mut Resource> {
        self.resources.get_mut(resource_id)
    }

    pub fn add(&mut self, resource: Resource) -> ResourceId {
        let id = nanoid::nanoid!();

        self.resources.insert(id.clone(), resource);
        id
    }

    pub fn reload_all(&mut self, parent: Option<&Path>) -> bool {
        let mut any_changes = false;
        for (_id, r) in self.resources.iter_mut() {
            any_changes |= r.reload(parent);
        }

        any_changes
    }
}
