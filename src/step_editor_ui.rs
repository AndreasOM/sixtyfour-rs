use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;
use crate::state::State;
use crate::step_editor_label::StepEditorLabel;
use crate::step_editor_program::StepEditorProgram;
use crate::step_editor_set_uniform_f32::StepEditorSetUniformF32;
use crate::StepEditor;
use crate::StepEditorScratch;

#[derive(Debug)]
pub struct StepEditorUi {
    step_editors: Vec<Box<dyn StepEditor>>,
}

impl Default for StepEditorUi {
    fn default() -> Self {
        let mut step_editors: Vec<Box<dyn StepEditor>> = Vec::default();
        step_editors.push(Box::new(StepEditorProgram::default()));
        step_editors.push(Box::new(StepEditorSetUniformF32::default()));
        step_editors.push(Box::new(StepEditorLabel::default()));

        Self { step_editors }
    }
}
impl StepEditorUi {
    pub fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step_editor_scratch: &mut StepEditorScratch,
        //project: &Project,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool {
        for e in &self.step_editors {
            if e.update(ui, project, step_editor_scratch, step, grid_pos) {
                return true;
            }
        }
        ui.label(format!("Unhandled Step {step:?}"));
        false
    }
}
