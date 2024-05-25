use egui::Response;
use egui::Ui;

use egui::Widget;

#[derive(Debug)]
pub struct UiGridCell {
	content: String,
}

impl Default for UiGridCell {
    fn default() -> Self {
        Self { 
        	content: Default::default(),
        }
    }
}

impl UiGridCell {
    pub fn new(content: String) -> Self {
        Self {
        	content,
            ..Default::default()
        }
    }
}

impl Widget for UiGridCell {
    fn ui(self, ui: &mut Ui) -> Response {
        let r = ui.available_size();

        let (rect, response) = ui.allocate_exact_size(r, egui::Sense::click());
        let visuals = ui.style().interact_selectable(&response, true);
        ui.painter().rect(
            rect,
            0.125 * rect.height(),
            visuals.bg_fill,
            visuals.bg_stroke,
        );

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER, // :TODO: decide
            self.content,
            egui::FontId::monospace( 12.0 ),
            egui::Color32::LIGHT_GRAY,
        );

        response
    }
}
