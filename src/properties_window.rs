use crate::property_ui::PropertyUi;
use crate::state::State;
use crate::window::Window;

#[derive(Debug, Default)]
pub struct PropertiesWindow {
    property_ui: PropertyUi,
}

impl Window for PropertiesWindow {
    fn name(&self) -> &str {
        "Properties"
    }
    fn is_open(&self) -> bool {
        true
    }
    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Properties")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                let enabled = true;
                if ui
                    .add_enabled(enabled, egui::Button::new("Wipe All!!!"))
                    .on_hover_text("Danger!!!")
                    .clicked()
                {
                    state.property_manager.wipe_all();
                }

                for (k, p) in state.property_manager.entries_mut().iter_mut() {
                    self.property_ui.property(ctx, ui, k, p);
                }
            });
        self.property_ui.update(ctx);
    }
}

impl PropertiesWindow {}
