use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::Project;
use crate::project::Step;
use crate::Command;
use crate::StepEditor;
use crate::StepEditorScratch;

#[derive(Debug, Default)]
pub struct StepEditorLabel {}

impl StepEditorLabel {}

impl StepEditor for StepEditorLabel {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step_editor_scratch: &mut StepEditorScratch,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool {
        match step {
            Step::Label { name, .. } => {
                ui.label("L");
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
                if committed {
                    let name = step_editor_scratch.string_mut("name", name).to_owned();
                    let _ = COMMAND_QUEUE.send(Command::HackStepLabelSetName {
                        grid_pos: grid_pos.clone(),
                        name: name.clone(),
                    });
                }

                true
            }
            _ => false,
        }
    }
}
