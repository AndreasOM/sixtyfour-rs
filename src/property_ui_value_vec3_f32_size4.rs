use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;
use egui::Color32;
use egui::RichText;
use egui::WidgetText;

#[derive(Debug, Default)]
pub struct PropertyUiValueVec3F32Size4 {}

impl PropertyUiValueVec3F32Size4 {}

impl PropertyUiValue for PropertyUiValueVec3F32Size4 {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText> {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32Size4 { values },
                PropertyConfig::F32 {
                    min_value: _,
                    max_value: _,
                    step_size: _,
                },
            ) => Some(
                format!(
                    "{name} {:.3}, {:.3}, {:.3}",
                    values[0], values[1], values[2]
                )
                .into(),
            ),
            (PropertyValue::Vec3F32Size4 { values }, PropertyConfig::ColorPal {}) => Some(
                format!(
                    "{name} {:.3}, {:.3}, {:.3}",
                    values[0], values[1], values[2]
                )
                .into(),
            ),
            _ => None,
        }
    }
    fn update(&self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32Size4 { values },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.vertical(|ui| {
                    //ui.label(name);
                    //egui::widgets::color_picker::color_edit_button_rgb( ui, &mut *values);

                    for i in 0..12 {
                        ui.add(
                            egui::Slider::new(&mut (*values)[i], *min_value..=*max_value)
                                .step_by(*step_size as f64)
                                .text(format!("{i}")),
                        );
                    }
                });
                true
            }
            (PropertyValue::Vec3F32Size4 { values }, PropertyConfig::ColorPal {}) => {
                ui.vertical(|ui| {
                    //ui.label(name);
                    //egui::widgets::color_picker::color_edit_button_rgb( ui, &mut *values);

                    egui::Grid::new(name).show(ui, |ui| {
                        ui.label("TL");
                        ui.vertical(|ui| {
                            for i in 6..=8 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.label("TR");
                        ui.end_row();
                        ui.horizontal(|ui| {
                            for i in 0..=2 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .vertical()
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.label("MIDDLE");
                        ui.horizontal(|ui| {
                            for i in 3..=5 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .vertical()
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.end_row();
                        ui.separator();
                        ui.separator();
                        ui.separator();
                        ui.end_row();
                        ui.label("BL");
                        ui.vertical(|ui| {
                            for i in 9..=11 {
                                // c - rgb
                                ui.add(
                                    egui::Slider::new(&mut (*values)[i], 0.0..=2.0)
                                        .step_by(0.1 as f64)
                                        .text(format!("{i}")),
                                );
                            }
                        });
                        ui.label("BR");
                    });
                });
                true
            }
            _ => false,
        }
    }
}
