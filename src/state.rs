use crate::project::Project;
use crate::project::ResourceId;
use std::collections::VecDeque;
use std::path::Path;

use std::path::PathBuf;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub project_path: Option<PathBuf>,
    #[serde(skip)]
    pub project: Project,

    #[serde(default)]
    recent_project_paths: VecDeque<PathBuf>,

    #[serde(default)]
    selected_program_id: Option<ResourceId>,
}

impl State {
    pub fn select_program_id(&mut self, id: ResourceId) {
        self.selected_program_id = Some(id);
    }
    pub fn deselect_program_id(&mut self) {
        self.selected_program_id = None;
    }
    pub fn selected_program_id(&self) -> Option<&ResourceId> {
        self.selected_program_id.as_ref()
    }
    pub fn project_path(&self) -> Option<&Path> {
        self.project_path.as_deref()
    }
    pub fn set_project_path(&mut self, project_path: PathBuf) {
        if let Some(old_project_path) = self.project_path.take() {
            self.recent_project_paths.push_back(old_project_path);
            while self.recent_project_paths.len() > 5 {
                self.recent_project_paths.pop_front();
            }
        }

        self.recent_project_paths.retain(|p| *p != project_path);

        self.project_path = Some(project_path);
    }

    pub fn recent_project_paths(&self) -> &VecDeque<PathBuf> {
        &self.recent_project_paths
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
                Ok(project) => {
                    self.project = project;
                }
                Err(_) => {
                    // :TODO: report
                }
            }
        }
    }

    pub fn clear_project(&mut self) {
        self.project = Project::default();
    }
}
