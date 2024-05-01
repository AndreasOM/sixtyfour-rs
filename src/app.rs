use crate::engine::McGuffin;
use crate::mc_guffin_window::McGuffinWindow;
use crate::resources_window::ResourcesWindow;

use crate::project_window::ProjectWindow;
use crate::properties_window::PropertiesWindow;

use crate::shaders_window::ShadersWindow;
use crate::state::State;
use crate::window::Window;
use egui::mutex::Mutex;
use egui::mutex::MutexGuard;
use std::sync::Arc;

#[derive(Default)]
struct McGuffinContainer(Arc<Mutex<McGuffin>>);
impl McGuffinContainer {
    pub fn lock(&mut self) -> MutexGuard<'_, McGuffin> {
        self.0.lock()
    }
    pub fn clone(&self) -> Arc<Mutex<McGuffin>> {
        Arc::clone(&self.0)
    }
}

impl serde::Serialize for McGuffinContainer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mg = self.0.lock();
        mg.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for McGuffinContainer {
    fn deserialize<D>(d: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mg = McGuffin::deserialize(d)?;
        Ok(Self(Arc::new(Mutex::new(mg))))
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    mc_guffin: McGuffinContainer,

    #[serde(skip)]
    windows: Vec<Box<dyn Window>>,

    //properties: HashMap<String, f32>,
    //#[serde(skip)]
    //property_manager: PropertyManager,
    state: State,

    #[serde(skip)]
    start_time: std::time::Instant,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            mc_guffin: Default::default(),
            windows: Default::default(),
            state: Default::default(),
            start_time: std::time::Instant::now(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //cc.egui_ctx.set_zoom_factor( 2.0 );

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let mut s: Self = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        s.state.reload_project();
        if let Some(get_proc_address) = cc.get_proc_address {
            let mgc = s.mc_guffin.clone();
            match s.mc_guffin.lock().setup(get_proc_address) {
                Ok(()) => {
                    s.windows.push(Box::new(McGuffinWindow::new(mgc.clone())));
                    s.windows.push(Box::new(ShadersWindow::new(mgc.clone())));
                }
                Err(e) => {
                    eprintln!("McGuffin setup error -> {e:#?}");
                }
            };
        }
        s.state
            .project
            .property_manager
            .ensure_all_properties_from_uniforms(s.mc_guffin.lock().uniform_manager());
        /*
                s.state
                    .property_manager
                    .ensure_property_f32("scale_red_x", 11.0);
                s.state
                    .property_manager
                    .ensure_property_f32("scale_green_y", 15.0);
                s.state.property_manager.ensure_property_f32("speed", 1.0);
        */
        s.start_time = std::time::Instant::now();

        // backfill properties as needed

        s.windows.push(Box::new(PropertiesWindow::default()));
        s.windows.push(Box::new(ProjectWindow::default()));
        s.windows.push(Box::new(ResourcesWindow::default()));

        s
    }
}

impl TemplateApp {}
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.state.save_project();

        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // McGuffin
        {
            let mut mg = self.mc_guffin.lock();
            mg.update_from_project(&self.state.project);

            let t = self.start_time.elapsed().as_secs_f32();
            mg.set_time(t);
        }

        for w in self.windows.iter_mut() {
            if w.is_open() {
                w.update(ctx, &mut self.state);
            }
        }

        ctx.request_repaint();
    }
}
