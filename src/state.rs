use crate::project;
use crate::project::Project;
use std::path::Path;

use std::path::PathBuf;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub project_path: Option<PathBuf>,
    #[serde(skip)]
    pub project: Project,
}

impl State {
    pub fn project_path(&self) -> Option<&Path> {
        self.project_path.as_deref()
    }
    pub fn set_project_path(&mut self, project_path: PathBuf) {
        self.project_path = Some(project_path);
    }

    pub fn save_project(&mut self) {
        // save the project
        if let Some(pp) = &self.project_path {
            match self.project.save(pp) {
                Ok(_) => {}
                Err(e) => {
                    // :TODO: report
                    eprintln!("Failed saving project {e:#?}")
                }
            }
        }
    }

    pub fn reload_project(&mut self) {
        if let Some(pp) = self.project_path() {
            match Project::try_load(pp) {
                Ok(project) => self.project = project,
                Err(_) => {
                    // :TODO: report
                }
            }
        }
    }
}
