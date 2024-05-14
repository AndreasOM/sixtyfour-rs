use crate::state::State;
use crate::window::Window;
use egui::Vec2b;
use egui_plot::Line;
use egui_plot::Plot;

#[derive(Debug, Default)]
pub struct PerformanceWindow {
    is_open: bool,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct PerformanceWindowSave {
    #[serde(default)]
    is_open: bool,
}

impl From<&PerformanceWindow> for PerformanceWindowSave {
    fn from(pw: &PerformanceWindow) -> Self {
        Self {
            is_open: pw.is_open,
        }
    }
}

impl Window for PerformanceWindow {
    fn name(&self) -> &str {
        "Performance"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Performance")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                let avg_d = state.paint_time_series().avg(20);
                let min_d = state.paint_time_series().min(20);
                let max_d = state.paint_time_series().max(20);
                ui.label(format!("{min_d} < {avg_d} < {max_d}"));
                let durations: Vec<_> = state
                    .paint_time_series()
                    .values()
                    .enumerate()
                    .map(|(i, v)| [i as f64, *v as f64])
                    .collect();

                let line = Line::new(durations);
                let points_60 = [[0.0f64, 16.6f64], [1000.0, 16.6]].to_vec();
                let line_60 = Line::new(points_60);
                Plot::new("paint_duration")
                    //.view_aspect(2.0)
                    //.clamp_grid( true )
                    .allow_scroll(false)
                    .allow_drag(false)
                    .auto_bounds(Vec2b::new(false, true))
                    .include_x(0.0)
                    .include_x(state.paint_time_series().count() as f32)
                    .include_y(0.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(line);
                        plot_ui.line(line_60);
                    });
            });
    }
    fn serialize(&self) -> String {
        let save: PerformanceWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let save: PerformanceWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
    }
}

impl PerformanceWindow {}
