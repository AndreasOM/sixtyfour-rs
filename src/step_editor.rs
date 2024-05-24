use crate::project::Project;
use crate::project::Step;

pub trait StepEditor: core::fmt::Debug {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step: &Step,
        block_idx: usize,
        step_idx: usize,
    ) -> bool;
}
