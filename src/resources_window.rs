use crate::project::Resource;
use crate::project::ResourceProgram;
use crate::project::ResourceText;
use crate::state::State;
use crate::window::Window;

#[derive(Debug, Default)]
pub struct ResourcesWindow {
    is_open: bool,
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
            .show(ctx, |ui| {
                let mut selected_program_id = None;
                let mut deselect_program_id = false;
                let current_selected_program_id = state.selected_program_id().cloned();
                for (id, r) in state.project.resource_manager.resources_mut() {
                    match r {
                        Resource::Text(rt) => {
                            ui.horizontal(|ui| {
                                let f = rt
                                    .file()
                                    .map(|f| format!("{f:?}"))
                                    .unwrap_or_else(|| Default::default());
                                ui.label(format!("  TXT {f}")).on_hover_text(id);
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
                                let s = if Some(id) == current_selected_program_id.as_ref() {
                                    //state.selected_program_id() {
                                    if ui.button("[D]").on_hover_text("Deselect").clicked() {
                                        deselect_program_id = true;
                                    }
                                    "!"
                                } else {
                                    if ui.button("[S]").on_hover_text("Select").clicked() {
                                        selected_program_id = Some(id.clone());
                                    }
                                    " "
                                };
                                ui.label(format!("{s} PRG")).on_hover_text(id);
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
}

impl ResourcesWindow {}
