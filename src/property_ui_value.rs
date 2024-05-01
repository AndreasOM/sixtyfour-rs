

use crate::project::Property;

pub trait PropertyUiValue: core::fmt::Debug {
    fn update(&self, ui: &mut egui::Ui, name: &str, property: &mut Property) -> bool;
}
