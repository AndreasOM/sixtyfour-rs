use crate::command_queue::COMMAND_QUEUE;
use crate::project::Property;
use crate::project::PropertyConfig;
use crate::property_ui_value_vec2_f32::PropertyUiValueVec2F32;
use crate::property_ui_value_vec3_f32::PropertyUiValueVec3F32;
use crate::property_ui_value_vec3_f32_size4::PropertyUiValueVec3F32Size4;
use crate::Command;
use egui::WidgetText;
use std::collections::HashMap;

use crate::property_ui_value_f32::PropertyUiValueF32;
use crate::PropertyUiValue;

#[derive(Debug)]
pub struct PropertyUi {
    configuring: Option<(String, PropertyConfig)>,
    applying: Option<(String, PropertyConfig)>,
    property_ui_values: Vec<Box<dyn PropertyUiValue>>,
    fully_closed: HashMap<String, bool>,
}

impl Default for PropertyUi {
    fn default() -> Self {
        let mut property_ui_values: Vec<Box<dyn PropertyUiValue>> = Vec::default();
        property_ui_values.push(Box::new(PropertyUiValueF32::default()));
        property_ui_values.push(Box::new(PropertyUiValueVec2F32::default()));
        property_ui_values.push(Box::new(PropertyUiValueVec3F32::default()));
        property_ui_values.push(Box::new(PropertyUiValueVec3F32Size4::default()));

        Self {
            configuring: Default::default(),
            applying: Default::default(),
            fully_closed: Default::default(),
            property_ui_values,
        }
    }
}
impl PropertyUi {
    pub fn update(&mut self, ctx: &egui::Context) {
        if let Some(c) = &mut self.configuring {
            let mut close = false;
            let mut cancel = false;
            let mut delete = false;
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
                        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                            if ui.add(egui::Button::new("Delete!!!")).clicked() {
                                delete = true;
                            }
                        });
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
                        ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                            if ui.add(egui::Button::new("Delete!!!")).clicked() {
                                delete = true;
                            }
                        });
                        let value = format!("Unhandled {:?}", c.1);
                        ui.label(value);
                        if ui.add(egui::Button::new("Cancel")).clicked() {
                            cancel = true;
                        }
                    }
                });
            if delete {
                let _ = COMMAND_QUEUE.send(Command::DeleteProperty {
                    name: c.0.to_owned(),
                });
                close = true;
            }
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

        let n = if true || *self.fully_closed.get(name).unwrap_or(&false) {
            let mut n = WidgetText::default();
            for v in self.property_ui_values.iter() {
                if let Some(l) = v.label(name, property) {
                    n = l;
                    break;
                }
            }
            //format!("{name} {n}")
            n
        } else {
            format!("{name}").into()
        };

        let n = n.monospace();

        let r = egui::CollapsingHeader::new(n)
            .id_source(name)
            .show(ui, |ui| {
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
            });
        if r.fully_closed() {
            self.fully_closed.insert(name.to_string(), true);
        } else {
            self.fully_closed.insert(name.to_string(), false);
        }

        if edit_clicked {
            self.configuring = Some((name.into(), property.config.clone()));
        }
    }
}
