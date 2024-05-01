use crate::state::State;
use crate::window::Window;

#[derive(Debug, Default)]
pub struct ProjectWindow {}

impl Window for ProjectWindow {
    fn name(&self) -> &str {
        "Project"
    }
    fn is_open(&self) -> bool {
        true
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
                if ui.button("Reload").clicked() {
                    state.reload_project();
                }
            });
    }
}

impl ProjectWindow {}
