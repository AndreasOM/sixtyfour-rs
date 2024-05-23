use crate::project::Resource;
use crate::project::ResourceId;
use color_eyre::Result;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct ResourceManager {
    resources: HashMap<ResourceId, Resource>,

    #[serde(skip)]
    version: u32,
}

impl ResourceManager {
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn dirty(&self) -> bool {
        self.resources.values().any(|r| r.dirty())
    }

    pub fn resources(&self) -> &HashMap<ResourceId, Resource> {
        &self.resources
    }
    pub fn resources_mut(&mut self) -> &mut HashMap<ResourceId, Resource> {
        &mut self.resources
    }

    pub fn with_resource_mut<F>(&mut self, resource_id: &ResourceId, mut f: F)
    where
        F: FnMut(&mut Resource) -> (),
    {
        if let Some(r) = self.resources.get_mut(resource_id) {
            let old_r_version = r.version();
            f(r);
            let new_r_version = r.version();

            if new_r_version != old_r_version {
                self.version += 1;
                eprintln!("ResourceManager version: {}", self.version);
            }
        }
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

    pub fn remove(&mut self, resource_id: &ResourceId) -> Option<Resource> {
        self.resources.remove(resource_id)
    }

    pub fn reload_all(&mut self, parent: Option<&Path>) -> bool {
        let mut any_changes = false;
        for (_id, r) in self.resources.iter_mut() {
            any_changes |= r.reload(parent);
        }

        any_changes
    }

    pub fn save_all(&mut self, parent: Option<&Path>) -> Result<()> {
        let mut any_error = false;
        for (id, r) in self.resources.iter_mut() {
            if let Err(e) = r.save(parent) {
                eprintln!("Error saving {id} -> {e:?}");
                any_error |= true;
            }
        }

        if any_error {
            eprintln!(":TODO: handle save errors");
        }

        Ok(())
    }
}
