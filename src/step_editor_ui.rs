use crate::project::Project;
use crate::project::Step;
use crate::step_editor_program::StepEditorProgram;
use crate::StepEditor;

#[derive(Debug)]
pub struct StepEditorUi {
    step_editors: Vec<Box<dyn StepEditor>>,
}

impl Default for StepEditorUi {
    fn default() -> Self {
        let mut step_editors: Vec<Box<dyn StepEditor>> = Vec::default();
        step_editors.push(Box::new(StepEditorProgram::default()));

        Self { step_editors }
    }
}
impl StepEditorUi {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step: &Step,
        block_idx: usize,
        step_idx: usize,
    ) -> bool {
        for e in &self.step_editors {
            if e.update(ui, project, step, block_idx, step_idx) {
                return true;
            }
        }
        ui.label(format!("Unhandled Step {step:?}"));
        false
    }
}