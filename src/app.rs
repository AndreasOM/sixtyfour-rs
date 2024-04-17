use crate::engine::McGuffin;

use core::ffi::c_void;
use core::ffi::CStr;

use egui::mutex::Mutex;
use std::sync::Arc;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    // *mut c_void, Option<extern "system" fn(A) -> Ret
    #[serde(skip)]
    gl_get_string: *const c_void,

    #[serde(skip)]
    mc_guffin: Arc<Mutex<McGuffin>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            gl_get_string: std::ptr::null_mut(),
            mc_guffin: Default::default(),
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

            //let name = c"glGetString";
            let name = CStr::from_bytes_with_nul(b"glGetString\0").unwrap();
            let get_string_addr = get_proc_address(name);
            s.gl_get_string = get_string_addr;
        }
        /*
                let gl = cc
                    .gl
                    .as_ref()
                    .expect("You need to run eframe with the glow backend");
        */
        s
    }
}

impl TemplateApp {
    fn mc_guffin_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, _sense) = ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
        let mc_guffin = self.mc_guffin.clone();
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
}
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /*
        match self.mc_guffin.update() {
            Ok(()) => {}
            Err(e) => {
                todo!("McGuffin update error -> {e:#?}");
            }
        };
        */

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.push_id("42", |ui| ui.label("The triangle is being painted using "));

                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });

            ui.push_id("mc_guffin", |ui| {
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.mc_guffin_painting(ui);
                });
            });
        });
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        /*
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            if let Some(gl) = frame.gl() {
                ui.horizontal(|ui| {
                    //let mut version;// = String::new();
                    //let mut version2;// = String::new();
                    let version = unsafe {
                        let s = gl.get_parameter_string(0x1F02);
                        format!("{s:?}")
                    };
                    let version2 = unsafe {
                        match transmute::<
                            *const c_void,
                            Option<extern "system" fn(c_uint) -> *const u8>,
                        >(self.gl_get_string)
                        {
                            Some(fn_p) => {
                                let result = fn_p(0x1F02);
                                let v = CStr::from_ptr(result as *const i8);
                                format!("{v:?}")
                            }
                            None => String::from("Can't get gl string"),
                        }
                    };
                    let s = format!("{version} == {version2}");
                    ui.label(s);
                });
            }
        });
        */
    }
}
