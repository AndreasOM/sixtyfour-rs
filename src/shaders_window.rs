use crate::engine::McGuffin;
use crate::state::State;
use crate::window::Window;
use color_eyre::Result;
use egui::mutex::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct ShadersWindow {
    mc_guffin: Arc<Mutex<McGuffin>>,
    active_shader_type: String,
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
                        let orig_shader_source =
                            mg.get_shader_source_source(&self.active_shader_type);
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

                    {
                        let mut mg = self.mc_guffin.lock();
                        if let Some(shader_source) =
                            mg.get_mut_shader_source(&self.active_shader_type)
                        {
                            let save_file = if let Some(save_path) = shader_source.save_path() {
                                save_path.to_string_lossy().to_string()
                            } else {
                                String::from("")
                            };
                            ui.label(save_file);
                        }
                    }

                    ui.horizontal_wrapped(|ui| {
                        let mut mg = self.mc_guffin.lock();
                        let enabled = dirty;
                        if ui
                            .add_enabled(enabled, egui::Button::new("Rebuild Program"))
                            .clicked()
                        {
                            let _ = mg.rebuild_program();
                            state
                                .project
                                .property_manager
                                .ensure_all_properties_from_uniforms(mg.uniform_manager());
                        }
                        if let Some(shader_source) =
                            mg.get_mut_shader_source(&self.active_shader_type)
                        {
                            let save_file = if let Some(save_path) = shader_source.save_path() {
                                save_path.to_string_lossy().to_string()
                            } else {
                                String::from("")
                            };

                            let enabled = shader_source.save_path().is_some();
                            if ui
                                .add_enabled(enabled, egui::Button::new("Save"))
                                .on_hover_text(&save_file)
                                .clicked()
                            {
                                let _ = shader_source.save();
                            }

                            if ui
                                .add_enabled(enabled, egui::Button::new("Reload"))
                                .on_hover_text(save_file)
                                .clicked()
                            {
                                let _ = shader_source.reload();
                            }

                            let enabled = true;
                            if ui
                                .add_enabled(enabled, egui::Button::new("Save as..."))
                                .clicked()
                            {
                                let filename = shader_source.default_file_name();
                                if let Some(file) = rfd::FileDialog::new()
                                    .set_directory(
                                        std::env::current_dir().unwrap_or_else(|_| "/".into()),
                                    )
                                    .set_file_name(filename)
                                    .save_file()
                                {
                                    shader_source.set_save_path(file);
                                    let _ = shader_source.save();
                                }
                            }
                            if ui
                                .add_enabled(enabled, egui::Button::new("Load from..."))
                                .clicked()
                            {
                                let filename = shader_source.default_file_name();
                                if let Some(file) = rfd::FileDialog::new()
                                    .set_directory(
                                        std::env::current_dir().unwrap_or_else(|_| "/".into()),
                                    )
                                    .set_file_name(filename)
                                    .pick_file()
                                {
                                    shader_source.set_save_path(file);
                                    let _ = shader_source.reload();
                                }
                            }
                        } // shader_source
                    });
                    ui.push_id("Compile Log", |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.label("Compile Log:");
                            let mg = self.mc_guffin.lock();
                            let ss = mg
                                .get_shader_source(&self.active_shader_type)
                                .expect("Shader should exist");
                            let compile_log = ss.compile_log();
                            for e in compile_log.iter() {
                                ui.label(e);
                            }
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut shader_source)
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
                            let mut mg = self.mc_guffin.lock();
                            mg.replace_shader_source(&self.active_shader_type, shader_source);
                        }
                    });
                }
            });
    }
}

impl ShadersWindow {
    pub fn new(mc_guffin: Arc<Mutex<McGuffin>>) -> Self {
        Self {
            mc_guffin,
            active_shader_type: String::from("fragment"),
        }
    }
}
