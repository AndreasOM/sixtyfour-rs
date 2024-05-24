use crate::command_queue::COMMAND_QUEUE;
use crate::project::Resource;
use crate::project::ResourceProgram;
use crate::project::ResourceText;
use crate::state::State;
use crate::window::Window;
use crate::Command;

#[derive(Debug, Default)]
pub struct ResourcesWindow {
    is_open: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct ResourcesWindowSave {
    #[serde(default)]
    is_open: bool,
}

impl From<&ResourcesWindow> for ResourcesWindowSave {
    fn from(rw: &ResourcesWindow) -> Self {
        Self {
            is_open: rw.is_open,
        }
    }
}

impl Window for ResourcesWindow {
    fn name(&self) -> &str {
        "Resources"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Resources")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                let mut selected_program_id = None;
                let mut deselect_program_id = false;
                let current_selected_program_id = state.selected_program_id().cloned();
                for (id, r) in state.project.resource_manager.resources_mut() {
                    match r {
                        Resource::Text(rt) => {
                            ui.horizontal(|ui| {
                                if ui.button("[X]").clicked() {
                                    let _ = COMMAND_QUEUE.send(Command::RemoveResource {
                                        resource_id: id.clone(),
                                    });
                                };

                                let f = rt
                                    .file()
                                    .map(|f| format!("{f:?}"))
                                    .unwrap_or_else(|| Default::default());
                                {
                                    let l = format!("    TXT {id} {f:20}"); // :TODO: truncate name?
                                                                            //eprintln!("{l}");
                                    let rt = egui::RichText::new(l).monospace();

                                    ui.label(rt);
                                }

                                //                                ui.label(format!("  TXT {f}")).on_hover_text(id);
                                let _response = ui.add(egui::TextEdit::singleline(rt.name_mut()));
                                /*
                                if response.changed() {
                                    // …
                                }
                                */
                            });
                        }
                        Resource::Program(rp) => {
                            ui.horizontal(|ui| {
                                let t = if Some(id) == current_selected_program_id.as_ref() {
                                    //state.selected_program_id() {
                                    if ui
                                        .button(egui::RichText::new("[D]").monospace())
                                        .on_hover_text("Deselect")
                                        .clicked()
                                    {
                                        deselect_program_id = true;
                                    }
                                    let l = format!(" ! PRG {id}                     ");
                                    egui::RichText::new(l).monospace().strong()
                                    //egui::RichText::new(format!("! PRG")).strong()
                                } else {
                                    if ui
                                        .button(egui::RichText::new("[S]").monospace())
                                        .on_hover_text("Select")
                                        .clicked()
                                    {
                                        selected_program_id = Some(id.clone());
                                    }
                                    let l = format!("   PRG {id}                     ");
                                    egui::RichText::new(l).monospace()
                                    //egui::RichText::new(format!(" PRG"))
                                };
                                ui.label(t).on_hover_text(id);
                                let _response = ui.add(egui::TextEdit::singleline(rp.name_mut()));
                            });
                        }
                        o => {
                            ui.label(format!("Unhandled {o:?}"));
                        }
                    }
                }
                if let Some(selected_program_id) = selected_program_id.take() {
                    state.select_program_id(selected_program_id);
                }
                if deselect_program_id {
                    state.deselect_program_id();
                }
                if ui.button("Add Text").clicked() {
                    let r = Resource::Text(ResourceText::default());

                    state.project.resource_manager.add(r);
                }
                if ui.button("Add Program").clicked() {
                    let r = Resource::Program(ResourceProgram::default());

                    state.project.resource_manager.add(r);
                }
            });
    }
    fn serialize(&self) -> String {
        let save: ResourcesWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let save: ResourcesWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
    }
}

impl ResourcesWindow {}
