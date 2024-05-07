use crate::property_ui::PropertyUi;
use crate::state::State;
use crate::window::Window;

#[derive(Debug, Default)]
pub struct PropertiesWindow {
    property_ui: PropertyUi,
    is_open: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct PropertiesWindowSave {
    #[serde(default)]
    is_open: bool,
}

impl From<&PropertiesWindow> for PropertiesWindowSave {
    fn from(pw: &PropertiesWindow) -> Self {
        Self {
            is_open: pw.is_open,
        }
    }
}

impl Window for PropertiesWindow {
    fn name(&self) -> &str {
        "Properties"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Properties")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                let enabled = true;
                if ui
                    .add_enabled(enabled, egui::Button::new("Wipe All!!!"))
                    .on_hover_text("Danger!!!")
                    .clicked()
                {
                    state.project.property_manager.wipe_all();
                }

                for (k, p) in state.project.property_manager.entries_mut().iter_mut() {
                    self.property_ui.property(ctx, ui, k, p);
                }
            });
        self.property_ui.update(ctx);
    }
    fn serialize(&self) -> String {
        let save: PropertiesWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let mut save: PropertiesWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
    }
}

impl PropertiesWindow {}
