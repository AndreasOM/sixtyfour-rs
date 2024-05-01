use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;

#[derive(Debug, Default)]
pub struct PropertyUiValueVec3F32 {}

impl PropertyUiValueVec3F32 {}

impl PropertyUiValue for PropertyUiValueVec3F32 {
    fn update(&self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool {
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
                    ui.label(name);
                    egui::widgets::color_picker::color_edit_button_rgb(ui, &mut *values);
                });
                true
            }
            _ => false,
        }
    }
}
