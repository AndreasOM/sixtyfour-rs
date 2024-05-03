use crate::state::State;
use crate::window::Window;

#[derive(Debug, Default)]
pub struct ProjectWindow {
    is_open: bool,
}

impl Window for ProjectWindow {
    fn name(&self) -> &str {
        "Project"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Project")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                ui.label("Project Path");
                let pp = if let Some(pp) = &state.project_path {
                    format!("{:?}", pp)
                } else {
                    String::new()
                };

                ui.label(pp);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Pick...").clicked() {
                        let current_dir = std::env::current_dir().unwrap_or_else(|_| "/".into());
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(state.project_path.as_ref().unwrap_or(&current_dir))
                            .pick_folder()
                        {
                            // :TODO: verify it is a project folder, or create a new project
                            state.set_project_path(path);
                        }
                    }
                    if ui.button("Load...").clicked() {
                        let current_dir = std::env::current_dir().unwrap_or_else(|_| "/".into());
                        if let Some(path) = rfd::FileDialog::new()
                            .set_directory(state.project_path.as_ref().unwrap_or(&current_dir))
                            .pick_folder()
                        {
                            // :TODO: verify it is a project folder, or create a new project
                            state.set_project_path(path);
                            state.reload_project();
                        }
                    }
                    if ui.button("Reload").clicked() {
                        state.reload_project();
                    }
                    if ui.button("Save").clicked() {
                        state.save_project();
                    }
                    if ui.button("New!").clicked() {
                        state.clear_project();
                    }
                });
                ui.label("Recent Projects");
                let mut picked = None;
                let mut load = false;
                for r in state.recent_project_paths().iter() {
                    ui.horizontal_wrapped(|ui| {
                        let rp = format!("{r:?}");
                        if ui.button("Pick").clicked() {
                            picked = Some(r.clone());
                        }
                        if ui.button("Load").clicked() {
                            picked = Some(r.clone());
                            load = true;
                        }
                        ui.label(rp);
                    });
                }
                if let Some(picked) = &picked {
                    state.set_project_path(picked.into());
                    if load {
                        state.reload_project();
                    }
                }
            });
    }
}

impl ProjectWindow {}
