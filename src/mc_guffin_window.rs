use crate::project::PropertyValue;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;

#[derive(Default)]
pub struct McGuffinWindow {
    is_open: bool,
}

impl core::fmt::Debug for McGuffinWindow {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct McGuffinWindowSave {
    #[serde(default)]
    is_open: bool,
}

impl From<&McGuffinWindow> for McGuffinWindowSave {
    fn from(mw: &McGuffinWindow) -> Self {
        Self {
            is_open: mw.is_open,
        }
    }
}

impl Window for McGuffinWindow {
    fn name(&self) -> &str {
        "McGuffin"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    fn serialize(&self) -> String {
        let save: McGuffinWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let mut save: McGuffinWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("McGuffin")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                self.mc_guffin_painting(ui, state);
            });
    }
}

impl McGuffinWindow {
    fn mc_guffin_painting(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let s = ui.available_size();

        let mut wanted_size = egui::Vec2::new(256.0, 144.0);
        let sx = s.x / wanted_size.x;
        let sy = s.y / wanted_size.y;

        let scale = sx.min(sy).max(1.0);
        wanted_size *= scale;

        let (rect, sense) = ui.allocate_at_least(wanted_size, egui::Sense::click_and_drag());
        if let Some(mc_guffin) = state.mc_guffin_cloned() {
            let callback = egui::PaintCallback {
                rect,
                callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                    move |_info, painter| {
                        mc_guffin.lock().paint(painter.gl());
                    },
                )),
            };
            ui.painter().add(callback);
        }
        if let Some(click_pos) = sense.interact_pointer_pos() {
            let rs = rect.max - rect.min;
            let np = ((click_pos - rect.min) / rs) * egui::Vec2::new(2.0, -2.0)
                + egui::Vec2::new(-1.0, 1.0);

            if let Some(p) = state.project.property_manager.get_mut("fMouseClick") {
                match p.value_mut() {
                    PropertyValue::Vec2F32 { ref mut values } => {
                        values[0] = np.x;
                        values[1] = np.y;
                    }
                    _ => {}
                }
            }
        }
        if let Some(click_pos) = sense.hover_pos() {
            let rs = rect.max - rect.min;
            let np = ((click_pos - rect.min) / rs) * egui::Vec2::new(2.0, -2.0)
                + egui::Vec2::new(-1.0, 1.0);

            if let Some(p) = state.project.property_manager.get_mut("fMouseHover") {
                match p.value_mut() {
                    PropertyValue::Vec2F32 { ref mut values } => {
                        values[0] = np.x;
                        values[1] = np.y;
                    }
                    _ => {}
                }
            }
        }
    }
}
