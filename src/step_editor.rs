use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;

pub trait StepEditor: core::fmt::Debug {
    fn update(&self, ui: &mut egui::Ui, project: &Project, step: &Step, grid_pos: &GridPos)
        -> bool;
}
