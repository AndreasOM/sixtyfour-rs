use crate::command_queue::COMMAND_QUEUE;
use crate::project::Step;
use crate::state::State;
use crate::window::Window;
use crate::Command;

#[derive(Debug, Default)]
pub struct FlowWindow {
    is_open: bool,
    selected_step: Option<(usize, usize)>,
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
                if let Some(selected_step) = &self.selected_step {
                    ui.label(format!("Selected {}-{}", selected_step.0, selected_step.1));
                    state.project.with_flow_mut(|flow| {
                        if let Some(b) = flow.blocks().get(selected_step.0) {
                            if let Some(s) = b.steps().get(selected_step.1) {
                                match s {
                                    Step::Program { resource_id, .. } => {
                                        ui.label("PR");
                                        ui.label("Resource Id");
                                        let mut r_id = resource_id.clone();
                                        let response =
                                            ui.add(egui::TextEdit::singleline(&mut r_id));
                                        if response.changed() {
                                            let _ = COMMAND_QUEUE.send(
                                                Command::HackChangeFlowProgramResourceId {
                                                    block_idx: selected_step.0,
                                                    step_idx: selected_step.1,
                                                    resource_id: r_id,
                                                },
                                            );
                                        }
                                    }
                                    _ => {
                                        ui.label(String::from(s));
                                    }
                                }
                            }
                        }
                    });
                }
                ui.separator();
                egui::ScrollArea::both().show(ui, |ui| {
                    egui::Grid::new("flow_grid").show(ui, |ui| {
                        state.project.with_flow_mut(|flow| {
                            for (b_idx, b) in flow.blocks().iter().enumerate() {
                                ui.label(String::from(b.name()));
                                ui.end_row();
                                for (s_idx, s) in b.steps().iter().enumerate() {
                                    if ui.label(String::from(s)).clicked() {
                                        self.selected_step = Some((b_idx, s_idx))
                                    };
                                    ui.end_row();
                                }
                            }
                        });
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