use crate::project::Property;
use egui::WidgetText;

pub trait PropertyUiValue: core::fmt::Debug {
    fn label(&self, name: &str, property: &mut Property) -> Option<WidgetText>;
    fn update(&mut self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool;
}
