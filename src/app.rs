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
use crate::WindowManager;
use crate::WindowsMenu;
use color_eyre::Result;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    //    #[serde(skip)]
    //    windows: Vec<Box<dyn Window>>,
    #[serde(skip)]
    window_manager: WindowManager,

    state: State,

    #[serde(skip)]
    start_time: std::time::Instant,

    #[serde(default)]
    windows_menu: Option<WindowsMenu>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            //windows: Default::default(),
            window_manager: Default::default(),
            state: Default::default(),
            start_time: std::time::Instant::now(),
            windows_menu: Default::default(),
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
            let mut s: Self = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();

            s.window_manager.add(Box::new(McGuffinWindow::default()));
            s.window_manager.add(Box::new(ShadersWindow::default()));
            s.window_manager.add(Box::new(PropertiesWindow::default()));
            s.window_manager.add(Box::new(ProjectWindow::default()));
            s.window_manager.add(Box::new(ResourcesWindow::default()));

            let app_save: AppSave =
                eframe::get_value(storage, &format!("{}-custom", eframe::APP_KEY))
                    .unwrap_or_default();
            let _ = s.apply_app_save(app_save);
            s
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

        s
    }
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct AppSave {
    window_manager: String,
}

impl AppSave {}

impl TemplateApp {
    fn apply_app_save(&mut self, app_save: AppSave) -> Result<()> {
        eprintln!("AppSave: {app_save:?}");
        self.window_manager.deserialize(&app_save.window_manager);
        Ok(())
    }
    fn as_app_save(&self) -> Result<AppSave> {
        let mut app_save = AppSave::default();
        app_save.window_manager = self.window_manager.serialize();

        Ok(app_save)
    }
}
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        self.state.save_project();

        eframe::set_value(storage, eframe::APP_KEY, self);

        match self.as_app_save() {
            Ok(app_save) => {
                eframe::set_value(storage, &format!("{}-custom", eframe::APP_KEY), &app_save)
            }
            Err(e) => {
                eprintln!("Failed saving custom AppSave {e:?}");
            }
        }
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
        egui::TopBottomPanel::top("menu_panel")
            //.resizable(true)
            //.min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Windows").clicked() {
                        if let Some(_windows_menu) = self.windows_menu.take() {
                        } else {
                            let wm = WindowsMenu::default();

                            self.windows_menu = Some(wm);
                        }
                    }
                });
            });

        if let Some(windows_menu) = &mut self.windows_menu {
            windows_menu.update(ctx, &mut self.state, &mut self.window_manager);
        }

        for w in self.window_manager.iter_mut() {
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
