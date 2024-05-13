use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;
use egui::Color32;
use egui::RichText;
use egui::WidgetText;

#[derive(Debug, Default)]
pub struct PropertyUiValueVec3F32 {}

impl PropertyUiValueVec3F32 {}

impl PropertyUiValue for PropertyUiValueVec3F32 {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText> {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32 { values },
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
            (PropertyValue::Vec3F32 { values }, PropertyConfig::ColorRgb {}) => {
                let c = Color32::from_rgb(
                    (values[0] * 255.0).floor() as u8,
                    (values[1] * 255.0).floor() as u8,
                    (values[2] * 255.0).floor() as u8,
                );
                let h = c.to_hex();
                Some(
                    RichText::new(format!(
                        "{name} {h} {:.3}, {:.3}, {:.3}",
                        values[0], values[1], values[2]
                    ))
                    .color(c)
                    // .monospace() // caller already ensures monospace
                    .into(),
                )
            }
            _ => None,
        }
    }
    fn update(&mut self, ui: &mut egui::Ui, _name: &str, property: &mut Property) -> bool {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec3F32 { values },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.vertical(|ui| {
                    //ui.label(name);
                    //egui::widgets::color_picker::color_edit_button_rgb( ui, &mut *values);

                    ui.add(
                        egui::Slider::new(&mut (*values)[0], *min_value..=*max_value)
                            .step_by(*step_size as f64)
                            .text("x"),
                    );
                    ui.add(
                        egui::Slider::new(&mut (*values)[1], *min_value..=*max_value)
                            .step_by(*step_size as f64)
                            .text("y"),
                    );
                    ui.add(
                        egui::Slider::new(&mut (*values)[2], *min_value..=*max_value)
                            .step_by(*step_size as f64)
                            .text("z"),
                    );
                });
                true
            }
            (PropertyValue::Vec3F32 { values }, PropertyConfig::ColorRgb {}) => {
                ui.vertical(|ui| {
                    //ui.label(name);
                    egui::widgets::color_picker::color_edit_button_rgb(ui, &mut *values);
                });
                true
            }
            _ => false,
        }
    }
}
