use crate::property_manager::Property;
use crate::property_manager::PropertyConfig;
use crate::property_manager::PropertyValue;

#[derive(Default, Debug, Clone)]
pub struct PropertyUi {
    configuring: Option<(String, PropertyConfig)>,
    applying: Option<(String, PropertyConfig)>,
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
                    _ => todo!(),
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

        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::F32 { value },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.horizontal_wrapped(|ui| {
                    {
                        ui.add(
                            egui::Slider::new(&mut *value, *min_value..=*max_value)
                                .step_by(*step_size as f64)
                                .text(name),
                        );
                    }
                    let enabled = self.configuring.is_none();
                    edit_clicked = ui
                        .add_enabled(enabled, egui::Button::new("⚙️ "))
                        .on_hover_text("Configure")
                        .clicked();
                });
            }
            _ => todo!(),
        }

        if edit_clicked {
            self.configuring = Some((name.into(), property.config.clone()));
        }
    }
}
