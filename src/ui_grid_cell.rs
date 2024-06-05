use egui::Response;
use egui::Ui;

use egui::Widget;

#[derive(Debug)]
pub struct UiGridCell {
    content: String,
    zoom: f32,
}

impl Default for UiGridCell {
    fn default() -> Self {
        Self {
            content: Default::default(),
            zoom: 1.0,
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
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
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
            egui::FontId::monospace(12.0 * self.zoom),
            egui::Color32::LIGHT_GRAY,
        );

        response
    }
}
