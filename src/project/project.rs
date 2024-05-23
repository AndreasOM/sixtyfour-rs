use crate::project::Block;
use crate::project::Flow;
use crate::project::PropertyManager;
use crate::project::ResourceManager;
use crate::project::Step;
use color_eyre::Result;
use std::path::Path;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    name: String,
    property_manager: PropertyManager,
    pub resource_manager: ResourceManager,
    //programs: HashMap<String, Program>,
    #[serde(default)]
    pub flow: Flow,

    #[serde(skip)]
    version: u32,
}

impl Project {
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn create_simple_flow(&mut self) {
        let mut flow = Flow::default();

        let mut block = Block::new(String::from("start"));
        block.add_step(Step::Program {
            resource_id: "Zm2nwFQDp3eQGuZldHMRA".into(),
        });
        block.add_step(Step::FullscreenQuad);

        flow.add_block(block);

        self.flow = flow;
    }

    pub fn with_property_manager<F>(&self, mut f: F)
    where
        F: FnMut(&PropertyManager) -> (),
    {
        f(&self.property_manager);
    }
    pub fn with_property_manager_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut PropertyManager) -> (),
    {
        let old_pm_version = self.property_manager.version();
        f(&mut self.property_manager);
        let new_pm_version = self.property_manager.version();

        if new_pm_version != old_pm_version {
            self.version += 1;
            eprintln!("Project version: {}", self.version);
        }
    }
    pub fn with_resource_manager<F>(&self, mut f: F)
    where
        F: FnMut(&ResourceManager) -> (),
    {
        f(&self.resource_manager);
    }
    pub fn with_resource_manager_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut ResourceManager) -> (),
    {
        let old_rm_version = self.resource_manager.version();
        f(&mut self.resource_manager);
        let new_rm_version = self.resource_manager.version();

        if new_rm_version != old_rm_version {
            self.version += 1;
            eprintln!("Project version: {}", self.version);
        }
    }
    pub fn dirty(&self) -> bool {
        self.resource_manager.dirty()
    }

    pub fn try_load(project_folder: &Path) -> Result<Self> {
        let mut filename = project_folder.to_path_buf();
        filename.push("sfrs.ron");

        let data = std::fs::read_to_string(&filename)?;

        let mut project: Project = ron::from_str(&data)?;
        project.version = 1;
        let _ = project.resource_manager.reload_all(Some(project_folder));
        Ok(project)
    }

    pub fn save(&mut self, filename: &Path) -> Result<()> {
        let mut filename = filename.to_path_buf();
        filename.push("sfrs.ron");

        let data = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::default()).unwrap();

        //println!("{data}" );
        std::fs::write(filename, data)?;
        Ok(())
    }

    pub fn save_all_resources(&mut self, project_folder: &Path) -> Result<()> {
        self.resource_manager.save_all(Some(project_folder))
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}
