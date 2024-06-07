use crate::command_queue::COMMAND_QUEUE;
use crate::project::GridPos;
use crate::project::GridRect;
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
    selected_grid_rect: Option<GridRect>,
    target_grid_pos: Option<GridPos>,
    target_grid_rect: Option<GridRect>,
    target_step_type: String,

    //#[serde(skip)]
    step_editor_ui: StepEditorUi,

    grid_zoom: f32,
    prevent_moving: bool,
    fixed_pos: Option<egui::Pos2>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
struct FlowWindowSave {
    #[serde(default)]
    is_open: bool,
    #[serde(default)]
    grid_zoom: f32,
}

impl From<&FlowWindow> for FlowWindowSave {
    fn from(pw: &FlowWindow) -> Self {
        Self {
            is_open: pw.is_open,
            grid_zoom: pw.grid_zoom,
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
        // :HACK: Cheat!
        if self.grid_zoom == 0.0 {
            self.grid_zoom = 1.0;
        }

        let mut w = egui::Window::new("Flow");
        if self.prevent_moving {
            if let Some(p) = self.fixed_pos {
                w = w.fixed_pos(p);
            }
        }

        if let Some(ir) = w
            .drag_to_scroll(!self.prevent_moving)
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
                        if let Some(selected_grid_rect) = &self.selected_grid_rect {
                            //ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                            let project = &mut state.project;
                            // let step_editor_scratch = &mut state.step_editor_scratch_mut();
                            let step_editor_scratch = &mut state.step_editor_scratch;
                            let (project, step_editor_scratch) =
                                state.project_and_step_editor_scratch_mut();
                            if let Some(s) =
                                project.flow().get_step_at(selected_grid_rect.top_left())
                            {
                                self.step_editor_ui.update(
                                    ui,
                                    project,
                                    step_editor_scratch,
                                    s,
                                    selected_grid_rect.top_left(),
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
                        ui.add(
                            egui::Slider::new(&mut self.grid_zoom, 0.125..=2.0)
                                .step_by(0.001)
                                .text("Grid Zoom"),
                        );
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

                        let mut new_target_grid_pos = self.target_grid_pos.clone();
                        let mut new_selected_grid_rect = self.selected_grid_rect.clone();
                        if ui.button("Add Step").clicked() {
                            if let Some(target_grid_pos) = &self.target_grid_pos {
                                let _ = COMMAND_QUEUE.send(Command::HackAddStepToFlow {
                                    grid_pos: target_grid_pos.clone(),
                                    step_type: self.target_step_type.clone(),
                                });
                                new_target_grid_pos.as_mut().unwrap().inc_y();
                            }
                        }
                        if ui.button("Remove Step").clicked() {
                            if let Some(target_grid_pos) = &self.target_grid_pos {
                                let _ = COMMAND_QUEUE.send(Command::HackRemoveStepFromFlow {
                                    grid_pos: target_grid_pos.clone(),
                                });
                                new_target_grid_pos.as_mut().unwrap().inc_y();
                            }
                        }
                        if ui.button("Move Step").clicked() {
                            if let (Some(target_grid_pos), Some(selected_grid_rect)) =
                                (&self.target_grid_pos, &self.selected_grid_rect)
                            {
                                let _ = COMMAND_QUEUE.send(Command::HackMoveStepInFlow {
                                    source_grid_pos: selected_grid_rect.top_left().clone(),
                                    target_grid_pos: target_grid_pos.clone(),
                                });
                                new_target_grid_pos.as_mut().unwrap().inc_y();
                                new_selected_grid_rect
                                    .as_mut()
                                    .unwrap()
                                    .top_left_mut()
                                    .inc_y();
                            }
                        }
                        if ui.button("Clone Step").clicked() {
                            if let (Some(target_grid_pos), Some(selected_grid_rect)) =
                                (&self.target_grid_pos, &self.selected_grid_rect)
                            {
                                let _ = COMMAND_QUEUE.send(Command::HackCloneStepInFlow {
                                    source_grid_pos: selected_grid_rect.top_left().clone(),
                                    target_grid_pos: target_grid_pos.clone(),
                                    overwrite: false,
                                });
                                new_target_grid_pos.as_mut().unwrap().inc_y();
                                new_selected_grid_rect
                                    .as_mut()
                                    .unwrap()
                                    .top_left_mut()
                                    .inc_y();
                            }
                        }

                        self.target_grid_pos = new_target_grid_pos;
                        self.selected_grid_rect = new_selected_grid_rect;
                    });
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    egui::ScrollArea::both()
                        .drag_to_scroll(!self.prevent_moving)
                        .show(ui, |ui| {
                            let mut grid = UiGrid::default();
                            grid.set_id(egui::Id::new("FlowGrid"));
                            grid.set_zoom(self.grid_zoom);
                            //grid.set_target_grid_pos(self.target_grid_pos.as_ref());
                            grid.set_target_rect(self.target_grid_rect.as_ref());

                            for (s, gp) in state.project.flow().steps().iter() {
                                grid.add_cell(gp.x(), gp.y(), UiGridCell::new(String::from(s)));
                            }

                            // grid.select_cell(self.selected_grid_rect.as_ref().map( |gr| gr.top_left() ) );
                            grid.select_rect(self.selected_grid_rect.as_ref());
                            if let Some(target_grid_pos) = &self.target_grid_pos {
                                grid.highlight_cell(&target_grid_pos);
                            }

                            let gr = grid.show(ui);
                            self.prevent_moving = gr.prevent_moving();
                            if gr.prevent_moving() {}

                            if let Some(gr) = gr.selected_grid_rect() {
                                // :TODO: only clear on change
                                //if self.selected_grid_pos != Some( *gp ) {
                                state.step_editor_scratch_mut().clear();
                                /*
                                let mut gr = GridRect::default();
                                gr.set_top_left( gp );
                                gr.set_size( &GridPos::new( 1, 1 ));
                                */
                                self.selected_grid_rect = Some(gr.clone());

                                //}
                            }

                            self.target_grid_pos = gr.target_grid_pos().cloned();
                            self.target_grid_rect = gr.target_grid_rect().cloned();
                        });
                });
            })
        {
            self.fixed_pos = Some(ir.response.rect.min.clone());
        }
    }
    fn serialize(&self) -> String {
        let save: FlowWindowSave = self.into();

        ron::ser::to_string(&save).unwrap_or_default()
    }
    fn deserialize(&mut self, data: &str) {
        let save: FlowWindowSave = ron::from_str(&data).unwrap_or_default();

        self.is_open = save.is_open;
        self.grid_zoom = save.grid_zoom;
    }
}

impl FlowWindow {}
