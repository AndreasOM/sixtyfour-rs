use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;
use crate::Command;
use crate::StepEditor;
use crate::StepEditorScratch;

#[derive(Debug, Default)]
pub struct StepEditorSetUniformF32 {}

impl StepEditorSetUniformF32 {}

impl StepEditor for StepEditorSetUniformF32 {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step_editor_scratch: &mut StepEditorScratch,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool {
        match step {
            Step::SetUniformF32 { name, value, .. } => {
                ui.label("SUF32");
                let mut committed = false;
                ui.horizontal(|ui| {
                    ui.label("name");
                    let name = step_editor_scratch.string_mut("name", name);
                    let response = ui.add(egui::TextEdit::singleline(name));
                    if response.lost_focus() {
                        // && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        eprintln!("Name changed to {name}");
                        committed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("value");
                    let value = step_editor_scratch.string_mut("value", value);
                    let response = ui.add(egui::TextEdit::singleline(value));
                    if response.lost_focus() {
                        // && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        eprintln!("Value changed to {value}");
                        committed = true;
                    }
                });
                if committed {
                    // HackStepSetUniformF32SteNameAndValue
                    let name = step_editor_scratch.string_mut("name", name).to_owned();
                    let value = step_editor_scratch.string_mut("value", value).to_owned();
                    let _ = COMMAND_QUEUE.send(Command::HackStepSetUniformF32SetNameAndValue {
                        grid_pos: grid_pos.clone(),
                        name: name.clone(),
                        value: value.clone(),
                    });
                }

                true
            }
            _ => false,
        }
    }
}
