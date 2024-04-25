use crate::engine::McGuffin;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;
use egui::mutex::Mutex;
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct McGuffinWindow {
    mc_guffin: Arc<Mutex<McGuffin>>,
}

impl core::fmt::Debug for McGuffinWindow {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Window for McGuffinWindow {
    fn name(&self) -> &str {
        "McGuffin"
    }
    fn is_open(&self) -> bool {
        true
    }
    fn update(&mut self, ctx: &egui::Context, _state: &mut State) {
        egui::Window::new("McGuffin")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                self.mc_guffin_painting(ui);
            });
    }
}

impl McGuffinWindow {
    /*
    pub fn setup( &mut self, mc_guffin: Arc<Mutex<McGuffin>> ) -> Result<()> {
        self.mc_guffin = Some( mc_guffin );
        Ok(())
    }
    */

    pub fn new(mc_guffin: Arc<Mutex<McGuffin>>) -> Self {
        Self { mc_guffin }
    }
    fn mc_guffin_painting(&mut self, ui: &mut egui::Ui) {
        let s = ui.available_size();

        let mut wanted_size = egui::Vec2::new(256.0, 144.0);
        let sx = s.x / wanted_size.x;
        let sy = s.y / wanted_size.y;

        let scale = sx.min(sy).max(1.0);
        wanted_size *= scale;

        let (rect, _sense) = ui.allocate_at_least(wanted_size, egui::Sense::drag());
        let mc_guffin = self.mc_guffin.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                move |_info, painter| {
                    mc_guffin.lock().paint(painter.gl());
                },
            )),
        };
        {
            /*
            // :TODO:
            let mut mg = mc_guffin.lock();

            for (k, v) in self.property_manager.entries_mut().iter_mut() {
                mg.set_property(k, *v);
            }

            let t = self.start_time.elapsed().as_secs_f32();
            mg.set_property("fTime", t);
            */
        }
        ui.painter().add(callback);
    }
}
