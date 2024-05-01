use crate::project::Program;
use std::collections::HashMap;
use crate::project::PropertyManager;
use color_eyre::Result;
use std::path::Path;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Project {
    name: String,
    pub property_manager: PropertyManager,
    programs: HashMap< String, Program>,
}

impl Project {
    pub fn try_load(filename: &Path) -> Result<Self> {
        let mut filename = filename.to_path_buf();
        filename.push("sfrs.ron");

        let data = std::fs::read_to_string(&filename)?;

        let project: Project = ron::from_str(&data)?;
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
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
}
