use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;
use crate::Command;
use crate::StepEditor;
use crate::StepEditorScratch;

#[derive(Debug, Default)]
pub struct StepEditorSetUniformVec3F32 {}

impl StepEditorSetUniformVec3F32 {}

impl StepEditor for StepEditorSetUniformVec3F32 {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step_editor_scratch: &mut StepEditorScratch,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool {
        match step {
            Step::SetUniformVec3F32 { name, values, .. } => {
                ui.label("SUFV3_32");
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
                    ui.label("values[0]");
                    let value = step_editor_scratch.string_mut("value_0", &values[ 0 ]);
                    let response = ui.add(egui::TextEdit::singleline(value));
                    if response.lost_focus() {
                        // && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        eprintln!("Value changed to {value}");
                        committed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("values[1]");
                    let value = step_editor_scratch.string_mut("value_1", &values[ 1 ]);
                    let response = ui.add(egui::TextEdit::singleline(value));
                    if response.lost_focus() {
                        // && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        eprintln!("Value changed to {value}");
                        committed = true;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("values[2]");
                    let value = step_editor_scratch.string_mut("value_2", &values[ 2 ]);
                    let response = ui.add(egui::TextEdit::singleline(value));
                    if response.lost_focus() {
                        // && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        eprintln!("Value changed to {value}");
                        committed = true;
                    }
                });
                if committed {
                    let name = step_editor_scratch.string_mut("name", name).to_owned();
                    let value_0 = step_editor_scratch.string_mut("value_0", &values[0]).to_owned();
                    let value_1 = step_editor_scratch.string_mut("value_1", &values[1]).to_owned();
                    let value_2 = step_editor_scratch.string_mut("value_2", &values[2]).to_owned();
                    let _ = COMMAND_QUEUE.send(Command::HackStepSetUniformVec3F32SetNameAndValues {
                        grid_pos: grid_pos.clone(),
                        name: name.clone(),
                        values: [value_0.clone(), value_1.clone(), value_2.clone() ],
                    });
                }

                true
            }
            _ => false,
        }
    }
}
