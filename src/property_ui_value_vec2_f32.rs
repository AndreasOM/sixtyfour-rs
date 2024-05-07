use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;
use egui::WidgetText;

#[derive(Debug, Default)]
pub struct PropertyUiValueVec2F32 {}

impl PropertyUiValueVec2F32 {}

impl PropertyUiValue for PropertyUiValueVec2F32 {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText> {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec2F32 { values },
                PropertyConfig::F32 {
                    min_value: _,
                    max_value: _,
                    step_size: _,
                },
            ) => Some(format!("{name} {:.3}, {:.3}", values[0], values[1]).into()),
            _ => None,
        }
    }
    fn update(&self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::Vec2F32 { values },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.vertical(|ui| {
                    ui.label(name);
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
                });
                true
            }
            (PropertyValue::Vec3F32 { values }, PropertyConfig::ColorRgb {}) => {
                ui.vertical(|ui| {
                    ui.label(name);
                    egui::widgets::color_picker::color_edit_button_rgb(ui, &mut *values);
                });
                true
            }
            _ => false,
        }
    }
}
