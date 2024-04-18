use crate::engine::McGuffin;
use egui::mutex::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    mc_guffin: Arc<Mutex<McGuffin>>,

    properties: HashMap<String, f32>,

    #[serde(skip)]
    start_time: std::time::Instant,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            mc_guffin: Default::default(),
            properties: Default::default(),
            start_time: std::time::Instant::now(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut s: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        if let Some(get_proc_address) = cc.get_proc_address {
            match s.mc_guffin.lock().setup(get_proc_address) {
                Ok(()) => {}
                Err(e) => {
                    todo!("McGuffin setup error -> {e:#?}");
                }
            };
        }
        s.ensure_property("scale_red_x", 11.0);
        s.ensure_property("scale_green_y", 15.0);
        s.ensure_property("speed", 1.0);

        s.start_time = std::time::Instant::now();

        // backfill properties as needed

        s
    }
}

impl TemplateApp {
    fn ensure_property(&mut self, name: &str, default_value: f32) {
        if !self.properties.contains_key(name) {
            self.properties.insert(name.into(), default_value);
        }
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
            let mut mg = self.mc_guffin.lock();
            for (k, v) in self.properties.iter_mut() {
                mg.set_property(k, *v);
            }

            let t = self.start_time.elapsed().as_secs_f32();
            mg.set_property("fTime", t);
        }
        ui.painter().add(callback);
    }
}
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let name = "McGuffin";
        egui::Window::new(name)
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                self.mc_guffin_painting(ui);
            });

        egui::Window::new("Properties")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                //self.mc_guffin_painting(ui);

                for (k, v) in self.properties.iter_mut() {
                    ui.add(egui::Slider::new(&mut *v, 0.0..=100.0).text(k));
                }
            });

        ctx.request_repaint();
    }
}
