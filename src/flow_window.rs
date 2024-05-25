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
                        if let Some(selected_step) = &self.selected_step {
                            //ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                            state.project.with_flow(|flow| {
                                if let Some(b) = flow.blocks().get(selected_step.0) {
                                    if let Some(s) = b.steps().get(selected_step.1) {
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

                        let x = 0;
                        let mut y = 0;
                        // :CHEAT: ???
                        let mut pos2step = HashMap::<(u16, u16), (usize, usize)>::new(); // ( x, y ) -> ( b_idx, s_idx );
                        for (b_idx, b) in state.project.flow().blocks().iter().enumerate() {
                            for (s_idx, s) in b.steps().iter().enumerate() {
                                //grid.add_cell(x, y, String::from(s));
                                grid.add_cell(x, y, UiGridCell::new(String::from(s)));
                                pos2step.insert((x, y), (b_idx, s_idx));
                                y += 1;
                            }
                        }

                        let gr = grid.show(ui);

                        if let Some((sx, sy)) = gr.selected() {
                            if let Some((b_idx, s_idx)) = pos2step.get(&(sx, sy)) {
                                //eprintln!("Selected {b_idx}, {s_idx}");
                                self.selected_step = Some((*b_idx, *s_idx))
                            }
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
