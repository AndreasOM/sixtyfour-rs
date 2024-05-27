use crate::engine::gl::Gl;
use crate::engine::ResourceLogManager;
use crate::engine::StepRunnerData;
use crate::engine::StepRunnerFullscreenQuad;
use crate::engine::StepRunnerProgram;
use crate::engine::StepRunnerSetUniformF32;
use crate::project::Flow;
use crate::project::Project;
use crate::project::Step;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct FlowVm {
    flow: Flow,
    step_runner_data: HashMap<String, Vec<Option<Box<dyn StepRunnerData>>>>,
}

impl FlowVm {
    pub fn load(&mut self, flow: &Flow) -> Result<()> {
        //    	eprintln!("Load!!!!!!!!!!!!!!!!!");
        self.flow = flow.clone();
        self.step_runner_data = HashMap::default();
        Ok(())
    }

    pub fn run_setup(
        &mut self,
        gl: &Gl,
        project: &Project,
        resource_log_manager: &mut ResourceLogManager,
    ) -> Result<()> {
        // !!! should only run once when project/flow is changed !!!
        if let Some(block) = self.flow.blocks().iter().find(|b| b.name() == "start") {
            let mut srd_block = Vec::with_capacity(block.steps_in_grid().len());
            srd_block.resize_with(block.steps_in_grid().len(), Default::default);

            for (s_idx, (step, _gp)) in block.steps_in_grid().iter().enumerate() {
                eprintln!("Setup {s_idx} {step:?}");
                match step {
                    Step::Program { .. } => {
                        let sr = StepRunnerProgram::default();
                        let mut srd = sr.create_data();
                        sr.run_setup(gl, project, resource_log_manager, step, &mut srd);
                        srd_block[s_idx] = srd;
                    }
                    Step::SetUniformF32 { .. } => {
                        let sr = StepRunnerSetUniformF32::default();
                        let mut srd = sr.create_data();
                        sr.run_setup(gl, step, &mut srd);
                        srd_block[s_idx] = srd;
                    }
                    Step::FullscreenQuad => {
                        let sr = StepRunnerFullscreenQuad::default();
                        let mut srd = sr.create_data();
                        sr.run_setup(gl, &mut srd);
                        srd_block[s_idx] = srd;
                    }
                    Step::Nop => {}
                }
            }
            self.step_runner_data
                .insert(String::from("start"), srd_block);
        } else {
            eprintln!("No `start` block in flow!");
        }
        Ok(())
    }
    pub fn run_update(&mut self, gl: &Gl) -> Result<()> {
        if let Some(block) = self.flow.blocks().iter().find(|b| b.name() == "start") {
            let srd_block = self
                .step_runner_data
                .get("start")
                .ok_or(eyre!("Data for block `start` not found"))?;
            for (s_idx, (step, _gp)) in block.steps_in_grid().iter().enumerate() {
                match step {
                    Step::Program { .. } => {
                        let sr = StepRunnerProgram::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, srd);
                    }
                    Step::SetUniformF32 { .. } => {
                        let sr = StepRunnerSetUniformF32::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, step, srd);
                    }
                    Step::FullscreenQuad => {
                        let sr = StepRunnerFullscreenQuad::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, srd);
                    }
                    Step::Nop => {}
                }
            }
        } else {
            eprintln!("No `start` block in flow!");
        }
        Ok(())
    }
}
