use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::Project;
use crate::project::Resource;
use crate::project::Step;
use crate::Command;
use crate::StepEditor;

#[derive(Debug, Default)]
pub struct StepEditorProgram {}

impl StepEditorProgram {}

impl StepEditor for StepEditorProgram {
    fn update(
        &self,
        ui: &mut egui::Ui,
        project: &Project,
        step: &Step,
        grid_pos: &GridPos,
    ) -> bool {
        match step {
            Step::Program { resource_id, .. } => {
                ui.label("PR");
                let r_name = project
                    .resource_manager()
                    .get(resource_id)
                    .map(|r| r.name())
                    .unwrap_or_default();
                ui.label(r_name);
                egui::ComboBox::from_label("Resource Id")
                    //.selected_text(format!("{}", resource_id))
                    .selected_text(
                        egui::RichText::new(format!("{resource_id}"))
                            .monospace()
                            .strong(),
                    )
                    .width(192.0)
                    .show_ui(ui, |ui| {
                        let program_resources = project
                            .resource_manager()
                            .resources()
                            .iter()
                            .filter(|(_k, r)| {
                                if let Resource::Program(_) = *r {
                                    true
                                } else {
                                    false
                                }
                            })
                            .map(|(k, r)| (k, r.name()));

                        let mut selected_resource_id = resource_id.clone();
                        for (id, name) in program_resources {
                            ui.selectable_value(
                                &mut selected_resource_id,
                                id.to_string(),
                                egui::RichText::new(format!("{id} - {name}")).monospace(),
                            );
                        }
                        if selected_resource_id != *resource_id {
                            let _ = COMMAND_QUEUE.send(Command::HackChangeFlowProgramResourceId {
                                grid_pos: grid_pos.clone(),
                                resource_id: selected_resource_id,
                            });
                        }
                    });

                true
            }
            _ => false,
        }
    }
}
