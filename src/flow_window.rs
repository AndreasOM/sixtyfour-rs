use crate::project::GridPos;
use crate::state::State;
use crate::step_editor_ui::StepEditorUi;
use crate::window::Window;
use crate::UiGrid;
use crate::UiGridCell;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FlowWindow {
    is_open: bool,
    selected_step: Option<(usize, usize)>,
    selected_grid_pos: Option<GridPos>,

    //#[serde(skip)]
    step_editor_ui: StepEditorUi,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct FlowWindowSave {
    #[serde(default)]
    is_open: bool,
}

impl From<&FlowWindow> for FlowWindowSave {
    fn from(pw: &FlowWindow) -> Self {
        Self {
            is_open: pw.is_open,
        }
    }
}

impl Window for FlowWindow {
    fn name(&self) -> &str {
        "Flow"
    }
    fn is_open(&self) -> bool {
        self.is_open
    }
    fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    fn update(&mut self, ctx: &egui::Context, state: &mut State) {
        egui::Window::new("Flow")
            .resizable(true)
            .hscroll(false)
            .vscroll(false)
            .collapsible(false)
            //.title_bar(false)
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                egui::TopBottomPanel::top("flow_top_panel")
                    //.resizable(true)
                    //.min_height(32.0)
                    .exact_height(128.0)
                    .show_inside(ui, |ui| {
                        if let Some(selected_grid_pos) = &self.selected_grid_pos {
                            //ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                            if let Some(s) = state.project.flow().get_step_at(selected_grid_pos) {
                                self.step_editor_ui.update(
                                    ui,
                                    &state.project,
                                    s,
                                    selected_grid_pos,
                                );
                            }
                            /*
                            state.project.with_flow(|flow| {
                                if let Some(b) = flow.blocks().get(selected_step.0) {
                                    if let Some((s, _gp)) = b.steps_in_grid().get(selected_step.1) {
                                        self.step_editor_ui.update(
                                            ui,
                                            &state.project,
                                            s,
                                            selected_step.0,
                                            selected_step.1,
                                        );
                                    }
                                }
                            });
                            */
                        }
                    });
                egui::TopBottomPanel::bottom("bottom_panel")
                    .resizable(false)
                    //.exact_height(16.0)
                    .min_height(16.0)
                    .show_inside(ui, |ui| {
                        if let Some(selected_step) = &self.selected_step {
                            ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                        }
                    });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        let mut grid = UiGrid::default();

                        for (_b_idx, b) in state.project.flow().blocks().iter().enumerate() {
                            for (_s_idx, (s, gp)) in b.steps_in_grid().iter().enumerate() {
                                grid.add_cell(gp.x(), gp.y(), UiGridCell::new(String::from(s)));
                            }
                        }

                        grid.select_cell(self.selected_grid_pos.as_ref());

                        let gr = grid.show(ui);

                        if let Some(gp) = gr.selected_grid_pos() {
                            self.selected_grid_pos = Some(gp.clone());
                        }
                    });
                });
            });
    }
    fn serialize(&self) -> String {
        let save: FlowWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let save: FlowWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
    }
}

impl FlowWindow {}
