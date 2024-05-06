use crate::project::Program;
use crate::project::PropertyManager;
use crate::project::ResourceManager;
use color_eyre::Result;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    name: String,
    pub property_manager: PropertyManager,
    pub resource_manager: ResourceManager,
    //programs: HashMap<String, Program>,
}

impl Project {
    pub fn try_load(project_folder: &Path) -> Result<Self> {
        let mut filename = project_folder.to_path_buf();
        filename.push("sfrs.ron");

        let data = std::fs::read_to_string(&filename)?;

        let mut project: Project = ron::from_str(&data)?;
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
