use crate::command_queue::COMMAND_QUEUE;
use crate::flow_window::FlowWindow;
use crate::mc_guffin_container::McGuffinContainer;
use crate::mc_guffin_window::McGuffinWindow;
use crate::performance_window::PerformanceWindow;
use crate::project::Resource;
use crate::project::Step;
use crate::project_window::ProjectWindow;
use crate::properties_window::PropertiesWindow;
use crate::resources_window::ResourcesWindow;
use crate::shaders_window::ShadersWindow;
use crate::state::State;
use crate::Command;
use crate::WindowManager;
use color_eyre::Result;
use egui::Color32;
use egui::RichText;

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
    //#[serde(skip)]
    // windows_menu: Option<WindowsMenu>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            //windows: Default::default(),
            window_manager: Default::default(),
            state: Default::default(),
            start_time: std::time::Instant::now(),
            // windows_menu: Default::default(),
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

            s.window_manager.add(Box::new(FlowWindow::default()));
            s.window_manager.add(Box::new(McGuffinWindow::default()));
            s.window_manager.add(Box::new(ShadersWindow::default()));
            s.window_manager.add(Box::new(PerformanceWindow::default()));
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
        let _todo_handle = self.state.save_all_resources();

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

    /*
    fn raw_input_hook(&mut self, _ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        eprintln!("{raw_input:?}")
    }
    */

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /* :TODO: :( doesn't work
        // quit handling
        ctx.input(|i|{
            if i.viewport().close_requested() {
                eprintln!("Close requested!");
                ctx.send_viewport_cmd( egui::viewport::ViewportCommand::CancelClose );
            }
        });
        */

        ctx.input_mut(|i| {
            if i.consume_key(egui::Modifiers::ALT, egui::Key::Enter) {
                //if i.consume_shortcut( &egui::KeyboardShortcut::new( egui::Modifiers::ALT, egui::Key::Enter ) ) {
                eprintln!("ALT-Enter");
                let _ = COMMAND_QUEUE.send(Command::ToggleFullscreen);
            }
        });

        // McGuffin
        {
            if let Some(mgc) = self.state.mc_guffin_cloned() {
                let mut mg = mgc.lock();
                mg.update_from_project(&self.state.project);

                /*
                self.state.project.with_property_manager_mut(|pm| {
                    pm.ensure_all_properties_from_uniforms(mg.uniform_manager());
                });
                */
                let t = self.start_time.elapsed().as_secs_f64();
                mg.set_time(t);
            }
        }
        if !self.state.mc_guffin_is_fullscreen {
            egui::TopBottomPanel::top("menu_panel")
                //.resizable(true)
                //.min_height(32.0)
                .show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("Open").clicked() {
                                // …
                            }
                        });
                        ui.menu_button("Windows", |ui| {
                            for w in self.window_manager.iter_mut() {
                                let name = if w.is_open() {
                                    let name = format!("*{}", w.name());
                                    egui::RichText::new(name).monospace().strong()
                                } else {
                                    let name = format!(" {}", w.name());
                                    egui::RichText::new(name).monospace()
                                };

                                if ui.button(name).clicked() {
                                    w.toggle();
                                }
                            }
                        });
                        if ui.button("Fullscreen").clicked() {
                            if self.state.mc_guffin_is_fullscreen {
                                self.state.mc_guffin_is_fullscreen = false;
                            } else {
                                self.state.mc_guffin_is_fullscreen = true;
                            }
                        }
                        ui.horizontal(|ui| {
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if let Some(pp) = self.state.project_path() {
                                        let dirty = self.state.project.dirty();
                                        let pp = pp.as_os_str().to_string_lossy();

                                        let c = if dirty {
                                            Color32::from_rgb(222, 144, 144)
                                        } else {
                                            Color32::from_rgb(222, 222, 222)
                                        };

                                        ui.label(
                                            RichText::new(format!("{pp}",)).color(c), // .monospace()
                                                                                      //.into(),
                                        );
                                    } else {
                                        ui.label("[no project]");
                                    }
                                    let d = if let Some(mgc) = self.state.mc_guffin() {
                                        let mg = mgc.lock();
                                        mg.last_paint_duration_in_ms()
                                    } else {
                                        0.0
                                    };

                                    self.state.paint_time_series_mut().push(d);
                                    let avg_d = self.state.paint_time_series().avg(20);
                                    let min_d = self.state.paint_time_series().min(20);
                                    let max_d = self.state.paint_time_series().max(20);
                                    ui.label(format!("{min_d} < {avg_d} < {max_d}"));
                                },
                            );
                        });
                    });
                });
            /*
                        if let Some(windows_menu) = &mut self.windows_menu {
                            windows_menu.update(ctx, &mut self.state, &mut self.window_manager);
                        }
            */
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
                    self.state.project.with_property_manager_mut(|pm| {
                        pm.delete_entry(&name);
                    });
                    // self.state.project.property_manager.delete_entry(&name);
                }
                Command::LeaveFullscreen => {
                    // :TODO: side effects
                    self.state.mc_guffin_is_fullscreen = false;
                }
                Command::ToggleFullscreen => {
                    // :TODO: side effects
                    self.state.mc_guffin_is_fullscreen = !self.state.mc_guffin_is_fullscreen;
                }
                Command::ProgramAddShader {
                    resource_id,
                    shader_type,
                    shader_resource_id,
                } => {
                    if let Some(resource) =
                        self.state.project.resource_manager.get_mut(&resource_id)
                    {
                        match resource {
                            Resource::Program(rp) => {
                                rp.add_shader(shader_type, shader_resource_id);
                            }
                            _ => {
                                eprintln!("{resource_id} is not a program!");
                            }
                        }
                    } else {
                        eprintln!("{resource_id} not found!");
                    }
                }
                Command::ProgramRemoveShader {
                    resource_id,
                    shader_resource_id,
                } => {
                    if let Some(resource) =
                        self.state.project.resource_manager.get_mut(&resource_id)
                    {
                        match resource {
                            Resource::Program(rp) => {
                                if rp.remove_shader(shader_resource_id) {
                                    // removed
                                } else {
                                    // not removed
                                }
                            }
                            _ => {
                                eprintln!("{resource_id} is not a program!");
                            }
                        }
                    } else {
                        eprintln!("{resource_id} not found!");
                    }
                }
                Command::RemoveResource { resource_id } => {
                    if let Some(_removed) = self.state.project.resource_manager.remove(&resource_id)
                    {
                        // removed
                    } else {
                        // not removed
                    }
                }
                Command::SelectProgram { resource_id } => {
                    self.state.select_program_id(resource_id);
                }
                Command::HackChangeFlowProgramResourceId {
                    grid_pos,
                    resource_id,
                } => {
                    //
                    self.state.project.with_flow_mut(|f| {
                        f.with_step_at_mut(&grid_pos, |step| {
                            let new_resource_id = &resource_id;
                            match step {
                                Step::Program {
                                    ref mut resource_id,
                                    ref mut version,
                                } => {
                                    eprintln!("Changing program to {resource_id}");
                                    *resource_id = new_resource_id.to_string();
                                    *version += 1;
                                }
                                _ => {}
                            }
                        });
                    })
                }
                Command::HackAddStepToFlow {
                    grid_pos,
                    step_type,
                } => {
                    //
                    self.state.project.with_flow_mut(|f| {
                        let step = Step::from(step_type.as_ref());
                        f.add_step(&grid_pos, step);
                    });
                }
                Command::HackRemoveStepFromFlow { grid_pos } => {
                    //
                    self.state.project.with_flow_mut(|f| {
                        f.remove_step(&grid_pos);
                    });
                }
                Command::HackMoveStepInFlow {
                    source_grid_pos,
                    target_grid_pos,
                } => {
                    //
                    self.state.project.with_flow_mut(|f| {
                        if let Some(step) = f.remove_step(&source_grid_pos) {
                            f.add_step(&target_grid_pos, step);
                        }
                    });
                }
                Command::HackCloneStepInFlow {
                    source_grid_pos,
                    target_grid_pos,
                    overwrite,
                } => {
                    //
                    self.state.project.with_flow_mut(|f| {
                        if let Some(step) = f.get_step_at(&source_grid_pos) {
                            if !overwrite & f.get_step_at(&target_grid_pos).is_some() {
                                eprintln!("Copy target already in use");
                            } else {
                                f.add_step(&target_grid_pos, step.clone());
                            }
                        }
                    });
                }
                Command::HackStepSetUniformF32SetNameAndValue {
                    grid_pos,
                    name,
                    value,
                } => {
                    self.state.project.with_flow_mut(|f| {
                        f.with_step_at_mut(&grid_pos, |s| {
                            let new_name = &name;
                            let new_value = &value;
                            match s {
                                Step::SetUniformF32 {
                                    name,
                                    value,
                                    version,
                                } => {
                                    *name = new_name.to_string();
                                    *value = new_value.to_string();
                                    *version += 1;
                                }
                                _ => {}
                            }
                        })
                    });
                }
                Command::HackStepSetUniformF64SetNameAndValue {
                    grid_pos,
                    name,
                    value,
                } => {
                    self.state.project.with_flow_mut(|f| {
                        f.with_step_at_mut(&grid_pos, |s| {
                            let new_name = &name;
                            let new_value = &value;
                            match s {
                                Step::SetUniformF64 {
                                    name,
                                    value,
                                    version,
                                } => {
                                    *name = new_name.to_string();
                                    *value = new_value.to_string();
                                    *version += 1;
                                }
                                _ => {}
                            }
                        })
                    });
                }
                Command::HackStepLabelSetName { grid_pos, name } => {
                    self.state.project.with_flow_mut(|f| {
                        f.with_step_at_mut(&grid_pos, |s| {
                            let new_name = &name;
                            match s {
                                Step::Label { name, version } => {
                                    *name = new_name.to_string();
                                    *version += 1;
                                }
                                _ => {}
                            }
                        })
                    });
                }
                o => {
                    eprintln!("Unhandled command {o:?}");
                }
            }
        }

        ctx.request_repaint();
    }
}
