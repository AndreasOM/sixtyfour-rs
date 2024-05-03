use crate::mc_guffin_container::McGuffinContainer;
use crate::project::Resource;
use crate::project::ResourceId;
use crate::project::ShaderType;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;

//#[derive(Clone)]
pub struct ShadersWindow {
    active_shader_type: String,

    active_resource_id: ResourceId,
    new_shader_type: ShaderType,
    new_shader_resource_id: ResourceId,
}

impl core::fmt::Debug for ShadersWindow {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl Window for ShadersWindow {
    fn name(&self) -> &str {
        "Properties"
    }
    fn is_open(&self) -> bool {
        true
    }
    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        let mgc = state.mc_guffin().map(|mgc| mgc.clone());
        egui::Window::new("Shaders")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .show(ctx, |ui| {
                if let Some( selected_program_id ) = state.selected_program_id().cloned() {
                    ui.label( format!("{selected_program_id}"));
                    let text_resources: Vec<ResourceId> = state.project.resource_manager.resources().iter().filter(|(_k,r)|{
                        if let Resource::Text(_) = *r  {
                            true
                        } else {
                            false
                        }
                    }).map(|(k,_r)| k.to_owned() ).collect();
                    if let Some( resource ) = state.project.resource_manager.get_mut( &selected_program_id ) {

                        match resource {
                            Resource::Program( rp ) => {
                                for s in rp.shaders() {
                                    ui.label(format!("{:?} {}", s.shader_type(), s.resource_id()));
                                }
                                let mut new_shader = None;
                                ui.horizontal(|ui|{
                                    if ui.button("Add Shader").clicked() {
                                        let shader_type = &self.new_shader_type;
                                        let resource_id = &self.new_shader_resource_id;
                                        new_shader = Some( ( *shader_type, resource_id.to_owned() ) );
                                    }
                                    let shader_type = &mut self.new_shader_type;
                                    egui::ComboBox::from_label("Shader Type")
                                        .selected_text(format!("{:?}", shader_type))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(shader_type, ShaderType::Fragment, "Fragment");
                                            ui.selectable_value(shader_type, ShaderType::Vertex, "Vertex");
                                        }
                                    );
                                    let resource_id = &mut self.new_shader_resource_id;
                                    egui::ComboBox::from_label("Resource Id")
                                        .selected_text(format!("{:?}", resource_id))
                                        .show_ui(ui, |ui| {
                                            for id in text_resources.iter() {
                                                ui.selectable_value(resource_id, id.to_string(), id);
                                            }
                                        }
                                    );
                                });

                                if let Some( ( shader_type, resource_id ) ) = new_shader.take() {
                                    rp.add_shader( shader_type, resource_id );
                                }
                                ui.separator();
                                ui.horizontal(|ui|{
                                    for s in rp.shaders() {
                                        //??? ui.visuals_mut().button_frame = false;

                                        let active = self.active_resource_id == *s.resource_id();
                                        let name = format!("{:?}", s.shader_type());
                                        if ui.add(egui::SelectableLabel::new(active, name)).clicked() {
                                            //self.active_shader_type = String::from(*st);
                                            self.active_resource_id = s.resource_id().to_owned();
                                        }
                                    }
                                });

                                ui.separator();

                                // editor
                                if let Some( r ) = state.project.resource_manager.get_mut( &self.active_resource_id ) {
                                    if let Resource::Text( rt ) = r {
                                        ui.horizontal_wrapped(|ui| {
                                            let enabled = true;
                                        if ui
                                            .add_enabled(enabled, egui::Button::new("Commit"))
                                            .clicked()
                                        {
                                            rt.commit_text_change();
                                        }

                                        let save_file = if let Some(save_path) = rt.file() {
                                            save_path.to_string_lossy().to_string()
                                        } else {
                                            String::from("")
                                        };
                                        let current_dir = std::env::current_dir().unwrap_or_else(|_| "/".into());


                                        let enabled = rt.file().is_some();
                                        if ui
                                            .add_enabled(enabled, egui::Button::new("Save"))
                                            .on_hover_text(&save_file)
                                            .clicked()
                                        {
                                            let _ = rt.save();
                                        }

                                        let enabled = true;
                                        if ui
                                            .add_enabled(enabled, egui::Button::new("Save as..."))
                                            .clicked()
                                        {
                                            //let filename = shader_source.default_file_name();
                                            if let Some(file) = rfd::FileDialog::new()
                                                .set_directory(
                                                    state.project_path.as_ref().unwrap_or(&current_dir)
                                                )
                                                //.set_file_name(filename)
                                                .save_file()
                                            {
                                                    rt.set_file( file );
                                                    let _ = rt.save();
                                            }
                                        }

                                        let enabled = rt.file().is_some();
                                            if ui
                                                .add_enabled(enabled, egui::Button::new("Reload"))
                                                .on_hover_text(save_file)
                                                .clicked()
                                            {
                                                let _ = rt.reload();
                                            }
                                        let enabled = true;
                                            if ui
                                                .add_enabled(enabled, egui::Button::new("Load from..."))
                                                .clicked()
                                            {
                                                //let filename = shader_source.default_file_name();
                                                if let Some(file) = rfd::FileDialog::new()
                                                    .set_directory(
                                                        state.project_path.as_ref().unwrap_or(&current_dir)
                                                    )
                                                    //.set_file_name(filename)
                                                    .pick_file()
                                                {
                                                    rt.set_file( file );
                                                    let _ = rt.reload();
                                                }
                                            }

                                        });

                                        ui.push_id("Compile Log", |ui| {
                                            egui::ScrollArea::vertical().show(ui, |ui| {
                                                if let Some( mgc ) = mgc {
                                                    ui.label("Compile Log:");
                                                    let mg = mgc.lock();
                                                    let compile_log = mg.get_resource_log( &self.active_resource_id );

                                                    for e in compile_log.iter() {
                                                        ui.label(e);
                                                    }
                                                }
                                            });
                                        });

                                        let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                                        let language = "c++";
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


                                        egui::ScrollArea::vertical().show(ui, |ui| {
                                            let response = ui.add(
                                                egui::TextEdit::multiline(rt.text_mut())
                                                    .code_editor()
                                                    .min_size(egui::Vec2::new(800.0, 500.0))
                                                    .layouter(&mut layouter)
                                                    .frame(true)
                                                    .desired_rows(80)
                                                    .desired_width(f32::INFINITY),
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
                                                // let mut mg = self.mc_guffin.lock();
                                                // mg.replace_shader_source(&self.active_shader_type, shader_source);
                                            }
                                        });

                                    }
                                }
                            },
                            _ => {
                                ui.label(format!("{selected_program_id} is not a program"));
                            }
                        }

                    } else {
                        // should *never* trigger
                        ui.label( "Selected program not found!" );    
                    }
                } else {
                    ui.label( "No program selected!" );
                }
                /*
                {
                    let mut theme =
                        egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
                    ui.collapsing("Theme", |ui| {
                        ui.group(|ui| {
                            theme.ui(ui);
                            theme.clone().store_in_memory(ui.ctx());
                        });
                    });
                }
                */
            });
    }
}

impl Default for ShadersWindow {
    fn default() -> Self {
        Self {
            active_shader_type: String::from("fragment"),

            active_resource_id: Default::default(),
            new_shader_type: ShaderType::Fragment,
            new_shader_resource_id: Default::default(),
        }
    }
}
