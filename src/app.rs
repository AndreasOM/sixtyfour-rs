use crate::engine::StoredMcGuffin;
use crate::engine::McGuffin;
use egui::mutex::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[serde(into = "StoredTemplateApp")]
#[serde(from = "StoredTemplateApp")]
#[derive(Clone)]
pub struct TemplateApp {
    #[serde(skip)]
    mc_guffin: Arc<Mutex<McGuffin>>,

    properties: HashMap<String, f32>,

    #[serde(skip)]
    start_time: std::time::Instant,

    #[serde(skip)]
    active_shader_type: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StoredTemplateApp {
    properties: HashMap<String, f32>,
    mc_guffin: StoredMcGuffin,
}

impl Into<StoredTemplateApp> for TemplateApp {

    fn into(self) -> StoredTemplateApp {
        let mg = self.mc_guffin.lock();
        let smg: StoredMcGuffin = StoredMcGuffin::from( &(*mg) );
        StoredTemplateApp {
            properties: self.properties.clone(),
            mc_guffin: smg,
        }
    }
}

impl From<StoredTemplateApp> for TemplateApp {

    fn from(sta: StoredTemplateApp) -> TemplateApp {
        //let mg = self.mc_guffin.lock();
        //let smg: StoredMcGuffin = StoredMcGuffin::from( &(*mg) );
        let mg = McGuffin::from( sta.mc_guffin );
        let mg = Arc::new( Mutex::new( mg ) );
        TemplateApp {
            properties: sta.properties.clone(),
            mc_guffin: mg,
            ..Default::default()
        }
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            mc_guffin: Default::default(),
            properties: Default::default(),
            start_time: std::time::Instant::now(),
            active_shader_type: String::from("fragment"),
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
                    eprintln!("McGuffin setup error -> {e:#?}");
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

        egui::Window::new("Shaders")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.visuals_mut().button_frame = false;
                    const SHADER_TYPES: &[&str] = &["vertex", "fragment"];

                    let mg = self.mc_guffin.lock();

                    for st in SHADER_TYPES {
                        let dirty = mg.is_shader_source_dirty(*st);
                        let active = self.active_shader_type == *st;
                        let name = if dirty {
                            format!("*{st}")
                        } else {
                            String::from(*st)
                        };
                        if ui.add(egui::SelectableLabel::new(active, name)).clicked() {
                            self.active_shader_type = String::from(*st);
                        }
                    }
                });
                {
                    let (mut shader_source, dirty) = {
                        let mg = self.mc_guffin.lock();
                        let orig_shader_source = mg.get_shader_source_source(&self.active_shader_type);
                        let dirty = mg.is_shader_source_dirty(&self.active_shader_type);
                        (String::from(orig_shader_source), dirty)
                    };

                    let mut theme =
                        egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                    ui.collapsing("Theme", |ui| {
                        ui.group(|ui| {
                            theme.ui(ui);
                            theme.clone().store_in_memory(ui.ctx());
                        });
                    });

                    let language = "c++";

                    //let shader_source = format!("{:#?}",)

                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        let mut layout_job = egui_extras::syntax_highlighting::highlight(
                            ui.ctx(),
                            &theme,
                            string,
                            language,
                        );
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    ui.horizontal_wrapped(|ui| {
                        let mut mg = self.mc_guffin.lock();
                        let enabled = dirty;
                        if ui
                            .add_enabled(enabled, egui::Button::new("Rebuild Program"))
                            .clicked()
                        {
                            let _ = mg.rebuild_program();
                        }
                        if let Some( shader_source ) = mg.get_mut_shader_source( &self.active_shader_type ) {
                            let save_file = if let Some( save_path ) = shader_source.save_path() {
                                save_path.to_string_lossy().to_string()
                            } else {
                                String::from("")
                            };

                            let enabled = shader_source.save_path().is_some();
                            if ui
                                .add_enabled(enabled, egui::Button::new("Save"))
                                .on_hover_text( save_file )
                                .clicked()
                            {
                                let _ = shader_source.save();
                            }

                            let enabled = true;
                            if ui
                                .add_enabled(enabled, egui::Button::new("Save as..."))
                                .clicked()
                            {
                                let filename = shader_source.default_file_name();
                                if let Some( file ) = rfd::FileDialog::new()
                                .set_directory( std::env::current_dir().unwrap_or_else(|_| "/".into() ) )
                                .set_file_name( filename )
                                .save_file() {

                                    shader_source.set_save_path( file );
                                    let _ = shader_source.save();
                                }
                            }

                        }
                    });
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut shader_source)
                                .code_editor()
                                .min_size(egui::Vec2::new(800.0, 500.0))
                                .layouter(&mut layouter)
                                .frame(true)
                                .desired_rows(80)
                                .desired_width(f32::INFINITY)
                                ,
                                /*
                                                   .font(egui::TextStyle::Monospace) // for cursor height
                                                   .code_editor()
                                                   .desired_rows(10)
                                                   .lock_focus(true)
                                                   .desired_width(f32::INFINITY)
                                                   .layouter(&mut layouter)
                                                   */
                        );

                        if response.changed() {
                            let mut mg = self.mc_guffin.lock();
                            mg.replace_shader_source(&self.active_shader_type, shader_source);
                        }
                    });
                }
            });

        ctx.request_repaint();
    }
}
