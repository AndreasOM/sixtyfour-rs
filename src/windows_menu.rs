use crate::state::State;
use crate::WindowManager;

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct WindowsMenu {}

impl WindowsMenu {
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        state: &mut State,
        window_manager: &mut WindowManager,
    ) {
        /*
        egui::SidePanel::left("windows_menu_panel").show(ctx, |ui| {
           ui.label("Hello World!");
        });
        */
        egui::Window::new("windows_menu_window")
            .resizable(false)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                for w in window_manager.iter_mut() {
                    let name = if w.is_open() {
                        format!("*{}", w.name())
                    } else {
                        format!("{}", w.name())
                    };
                    if ui.button(name).clicked() {
                        w.toggle();
                    }
                    /*
                    if w.is_open() {
                        w.update(ctx, &mut self.state);
                    }
                    */
                }
                ui.label("Shaders");
                ui.label("Properties");
            });
    }
}
