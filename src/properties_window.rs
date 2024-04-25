use crate::engine::McGuffin;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;
use egui::mutex::Mutex;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct PropertiesWindow {}

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
                //self.mc_guffin_painting(ui);

                for (k, v) in state.property_manager.entries_mut().iter_mut() {
                    ui.add(egui::Slider::new(&mut *v, 0.0..=100.0).text(k));
                }
            });
    }
}

impl PropertiesWindow {}
