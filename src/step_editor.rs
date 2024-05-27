use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;
use crate::StepEditorScratch;

pub trait StepEditor: core::fmt::Debug {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step_editor_scratch: &mut StepEditorScratch,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool;
}
