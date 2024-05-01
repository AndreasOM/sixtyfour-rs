use crate::project::Property;
use crate::project::PropertyConfig;
use crate::property_ui_value_vec3_f32::PropertyUiValueVec3F32;

use crate::property_ui_value_f32::PropertyUiValueF32;
use crate::PropertyUiValue;

#[derive(Debug)]
pub struct PropertyUi {
    configuring: Option<(String, PropertyConfig)>,
    applying: Option<(String, PropertyConfig)>,
    property_ui_values: Vec<Box<dyn PropertyUiValue>>,
}

impl Default for PropertyUi {
    fn default() -> Self {
        let mut property_ui_values: Vec<Box<dyn PropertyUiValue>> = Vec::default();
        property_ui_values.push(Box::new(PropertyUiValueF32::default()));
        property_ui_values.push(Box::new(PropertyUiValueVec3F32::default()));

        Self {
            configuring: Default::default(),
            applying: Default::default(),
            property_ui_values,
        }
    }
}
impl PropertyUi {
    pub fn update(&mut self, ctx: &egui::Context) {
        if let Some(c) = &mut self.configuring {
            let mut close = false;
            let mut cancel = false;
            let name = format!("Property Configuration: '{}'", c.0);
            egui::Window::new(name)
                .resizable(true)
                .hscroll(false)
                .vscroll(false)
                .collapsible(false)
                //.title_bar(false)
                .show(ctx, |ui| match &mut c.1 {
                    PropertyConfig::F32 {
                        min_value,
                        max_value,
                        step_size,
                    } => {
                        ui.add(
                            egui::Slider::new(&mut *min_value, 0.0..=100.0)
                                .clamp_to_range(false)
                                .text("Min"),
                        );
                        ui.add(
                            egui::Slider::new(&mut *max_value, 0.0..=100.0)
                                .clamp_to_range(false)
                                .text("Max"),
                        );
                        ui.add(
                            egui::Slider::new(&mut *step_size, 0.1..=100.0)
                                .clamp_to_range(false)
                                .text("Step Size"),
                        );
                        ui.horizontal_wrapped(|ui| {
                            if ui.add(egui::Button::new("Cancel")).clicked() {
                                cancel = true;
                            }
                            if ui.add(egui::Button::new("Apply")).clicked() {
                                close = true;
                            }
                        });
                    }
                    _ => {
                        let value = format!("Unhandled {:?}", c.1);
                        ui.label(value);
                        if ui.add(egui::Button::new("Cancel")).clicked() {
                            cancel = true;
                        }
                    }
                });
            if close {
                self.applying = self.configuring.take();
            }
            if cancel {
                self.configuring = None;
            }
        }
    }
    pub fn property(
        &mut self,
        _ctx: &egui::Context,
        ui: &mut egui::Ui,
        name: &str,
        property: &mut Property,
    ) {
        let mut edit_clicked = false;
        if let Some(applying) = &self.applying {
            //eprintln!("Trying to apply {applying:?}");
            if applying.0 == name {
                property.config = applying.1.clone();
                self.applying = None;
            }
        }

        ui.horizontal_wrapped(|ui| {
            let mut handled = false;

            for v in self.property_ui_values.iter() {
                if v.update(ui, name, property) {
                    handled = true;
                    break;
                }
            }

            if !handled {
                let value = format!("Unhandled {:?}", property.value);
                ui.label(value);
            }
            let enabled = self.configuring.is_none();
            edit_clicked = ui
                .add_enabled(enabled, egui::Button::new("⚙️ "))
                .on_hover_text("Configure")
                .clicked();
        });

        if edit_clicked {
            self.configuring = Some((name.into(), property.config.clone()));
        }
    }
}
