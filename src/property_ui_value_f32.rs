use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;
use egui::WidgetText;

#[derive(Debug, Default)]
pub struct PropertyUiValueF32 {}

impl PropertyUiValueF32 {}

impl PropertyUiValue for PropertyUiValueF32 {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText> {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::F32 { value },
                PropertyConfig::F32 {
                    min_value: _,
                    max_value: _,
                    step_size: _,
                },
            ) => Some(format!("{name} {value:.3}").into()),
            _ => None,
        }
    }
    fn update(&self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool {
        match (&mut property.value, &mut property.config) {
            (
                PropertyValue::F32 { value },
                PropertyConfig::F32 {
                    min_value,
                    max_value,
                    step_size,
                },
            ) => {
                ui.add(
                    egui::Slider::new(&mut *value, *min_value..=*max_value)
                        .step_by(*step_size as f64)
                        .text(name),
                );
                true
            }
            _ => false,
        }
    }
}
