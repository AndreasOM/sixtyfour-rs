use crate::command_queue::COMMAND_QUEUE;
use crate::mc_guffin_container::McGuffinContainer;
use crate::mc_guffin_window::McGuffinWindow;
use crate::project_window::ProjectWindow;
use crate::properties_window::PropertiesWindow;
use crate::resources_window::ResourcesWindow;
use crate::shaders_window::ShadersWindow;
use crate::state::State;
use crate::window::Window;
use crate::Command;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)]
    windows: Vec<Box<dyn Window>>,

    state: State,

    #[serde(skip)]
    start_time: std::time::Instant,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
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
            let mgc = McGuffinContainer::default();
            //let mgc = s.mc_guffin.clone();
            let mgc2 = mgc.clone();
            match mgc.lock().setup(get_proc_address) {
                Ok(()) => {
                    s.state.set_mc_guffin(mgc2);
                }
                Err(e) => {
                    eprintln!("McGuffin setup error -> {e:#?}");
                }
            };
        }
        /*
        // not program load at this point, so no uniforms to create properties from
        if let Some( mgc ) = s.state.mc_guffin_cloned() {
            eprintln!("Ensuring all properties");
            s.state
                .project
                .property_manager
                .ensure_all_properties_from_uniforms(mgc.lock().uniform_manager());
        }
        */

        s.start_time = std::time::Instant::now();

        s.windows.push(Box::new(McGuffinWindow::default()));
        s.windows.push(Box::new(ShadersWindow::default()));
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
            if let Some(mgc) = self.state.mc_guffin_cloned() {
                let mut mg = mgc.lock();
                mg.update_from_project(&self.state.project);

                self.state
                    .project
                    .property_manager
                    .ensure_all_properties_from_uniforms(mg.uniform_manager());

                let t = self.start_time.elapsed().as_secs_f32();
                mg.set_time(t);
            }
        }

        for w in self.windows.iter_mut() {
            if w.is_open() {
                w.update(ctx, &mut self.state);
            }
        }

        // handle pending commands
        while let Some(command) = COMMAND_QUEUE.next() {
            match command {
                Command::DeleteProperty { name } => {
                    self.state.project.property_manager.delete_entry(&name);
                }
                o => {
                    eprintln!("Unhandled command {o:?}");
                }
            }
        }

        ctx.request_repaint();
    }
}
