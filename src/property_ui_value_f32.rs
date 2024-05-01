use crate::project::Property;
use crate::project::PropertyConfig;
use crate::project::PropertyValue;
use crate::PropertyUiValue;

#[derive(Debug, Default)]
pub struct PropertyUiValueF32 {}

impl PropertyUiValueF32 {}

impl PropertyUiValue for PropertyUiValueF32 {
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
