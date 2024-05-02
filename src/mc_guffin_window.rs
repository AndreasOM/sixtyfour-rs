use crate::mc_guffin_container::McGuffinContainer;
use crate::project::PropertyValue;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;

#[derive(Default)]
pub struct McGuffinWindow {
    //mc_guffin: Arc<Mutex<McGuffin>>,
    mc_guffin: McGuffinContainer,
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
    /*
    pub fn setup( &mut self, mc_guffin: Arc<Mutex<McGuffin>> ) -> Result<()> {
        self.mc_guffin = Some( mc_guffin );
        Ok(())
    }
    */

    //    pub fn new(mc_guffin: Arc<Mutex<McGuffin>>) -> Self {
    pub fn new(mc_guffin: McGuffinContainer) -> Self {
        Self { mc_guffin }
    }
    fn mc_guffin_painting(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let s = ui.available_size();

        let mut wanted_size = egui::Vec2::new(256.0, 144.0);
        let sx = s.x / wanted_size.x;
        let sy = s.y / wanted_size.y;

        let scale = sx.min(sy).max(1.0);
        wanted_size *= scale;

        let (rect, sense) = ui.allocate_at_least(wanted_size, egui::Sense::click());
        let mc_guffin = self.mc_guffin.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(
                move |_info, painter| {
                    mc_guffin.lock().paint(painter.gl());
                },
            )),
        };
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
