use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::Step;
use crate::state::State;
use crate::step_editor_ui::StepEditorUi;
use crate::window::Window;
use crate::Command;
use crate::UiGrid;
use crate::UiGridCell;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FlowWindow {
    is_open: bool,
    selected_grid_pos: Option<GridPos>,
    target_grid_pos: Option<GridPos>,
    target_step_type: String,

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
                            let project = &mut state.project;
                            // let step_editor_scratch = &mut state.step_editor_scratch_mut();
                            let step_editor_scratch = &mut state.step_editor_scratch;
                            let (project, step_editor_scratch) =
                                state.project_and_step_editor_scratch_mut();
                            if let Some(s) = project.flow().get_step_at(selected_grid_pos) {
                                self.step_editor_ui.update(
                                    ui,
                                    project,
                                    step_editor_scratch,
                                    s,
                                    selected_grid_pos,
                                );
                            }
                        }
                        ui.separator();
                    });
                egui::TopBottomPanel::bottom("bottom_panel")
                    .resizable(false)
                    //.exact_height(16.0)
                    .min_height(16.0)
                    .show_inside(ui, |ui| {
                        /*
                        if let Some(selected_step) = &self.selected_step {
                            ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                        }
                        */
                    });
                egui::SidePanel::left("left_panel")
                    .resizable(false)
                    //.exact_height(16.0)
                    .min_width(128.0)
                    .show_inside(ui, |ui| {
                        ui.label("Grid Stuff");
                        /*
                        ui.add(
                            egui::DragValue::new(self.target_grid_pos.x_mut())
                                .speed(0.2)
                                .clamp_range(0..=31),
                        );
                        ui.add(
                            egui::DragValue::new(self.target_grid_pos.y_mut())
                                .speed(0.2)
                                .clamp_range(0..=31),
                        );
                        */

                        egui::ComboBox::from_label("Step Type")
                            .selected_text(
                                egui::RichText::new(format!("{}", self.target_step_type))
                                    .monospace()
                                    .strong(),
                            )
                            .width(128.0)
                            .show_ui(ui, |ui| {
                                for t in Step::types() {
                                    ui.selectable_value(
                                        &mut self.target_step_type,
                                        String::from(*t),
                                        *t,
                                    );
                                }
                            });
                        if ui.button("Add Step").clicked() {
                            if let Some(target_grid_pos) = &self.target_grid_pos {
                                let _ = COMMAND_QUEUE.send(Command::HackAddStepToFlow {
                                    grid_pos: target_grid_pos.clone(),
                                    step_type: self.target_step_type.clone(),
                                });
                            }
                        }
                        if ui.button("Remove Step").clicked() {
                            if let Some(target_grid_pos) = &self.target_grid_pos {
                                let _ = COMMAND_QUEUE.send(Command::HackRemoveStepFromFlow {
                                    grid_pos: target_grid_pos.clone(),
                                });
                            }
                        }
                        if ui.button("Move Step").clicked() {
                            if let (Some(target_grid_pos), Some(selected_grid_pos)) =
                                (&self.target_grid_pos, &self.selected_grid_pos)
                            {
                                let _ = COMMAND_QUEUE.send(Command::HackMoveStepInFlow {
                                    source_grid_pos: selected_grid_pos.clone(),
                                    target_grid_pos: target_grid_pos.clone(),
                                });
                            }
                        }
                    });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    egui::ScrollArea::both().show(ui, |ui| {
                        let mut grid = UiGrid::default();
                        grid.set_target_grid_pos(self.target_grid_pos.as_ref());

                        for (_b_idx, b) in state.project.flow().blocks().iter().enumerate() {
                            for (_s_idx, (s, gp)) in b.steps_in_grid().iter().enumerate() {
                                grid.add_cell(gp.x(), gp.y(), UiGridCell::new(String::from(s)));
                            }
                        }

                        grid.select_cell(self.selected_grid_pos.as_ref());
                        if let Some(target_grid_pos) = &self.target_grid_pos {
                            grid.highlight_cell(&target_grid_pos);
                        }

                        let gr = grid.show(ui);

                        if let Some(gp) = gr.selected_grid_pos() {
                            // :TODO: only clear on change
                            //if self.selected_grid_pos != Some( *gp ) {
                            state.step_editor_scratch_mut().clear();
                            self.selected_grid_pos = Some(gp.clone());

                            //}
                        }

                        self.target_grid_pos = gr.target_grid_pos().cloned();
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
