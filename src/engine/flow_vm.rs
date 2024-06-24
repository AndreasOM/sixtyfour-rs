use crate::engine::gl::Gl;
use crate::engine::ResourceLogManager;
use crate::engine::StepRunnerData;
use crate::engine::StepRunnerFullscreenQuad;
use crate::engine::StepRunnerProgram;
use crate::engine::StepRunnerSetUniformF32;
use crate::engine::StepRunnerSetUniformF64;
use crate::engine::StepRunnerSetUniformVec3F32;
use crate::project::Flow;
use crate::project::Project;
use crate::project::Step;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct FlowVm {
    flow: Flow,
    step_runner_data: HashMap<String, Vec<Option<Box<dyn StepRunnerData>>>>,
    start_time: std::time::Instant,
    time: f64,
}

impl Default for FlowVm {
    fn default() -> Self {
        Self {
            flow: Flow::default(),
            step_runner_data: HashMap::default(),
            start_time: std::time::Instant::now(),
            time: f64::default(),
        }
    }
}

impl FlowVm {
    pub fn time(&self) -> f64 {
        self.time
    }
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
        if let Some(start_step) = self.flow.steps().iter().find(|(s, _gp)| {
            if let Step::Label { name, .. } = s {
                name == "start"
            } else {
                false
            }
        }) {
            let (_s, pos) = start_step;
            // eprintln!("Starting at {pos:?}");
            let mut pos = pos.clone();
            let mut s_idx = 0;
            let mut srd_block = Vec::with_capacity(self.flow.steps().len());
            srd_block.resize_with(self.flow.steps().len(), Default::default);

            while let Some(step) = self.flow.get_step_at(&pos) {
                eprintln!("Setup [{pos:?}] {s_idx} {step:?}");
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
                    Step::SetUniformF64 { .. } => {
                        let sr = StepRunnerSetUniformF64::default();
                        let mut srd = sr.create_data();
                        sr.run_setup(gl, step, &mut srd);
                        srd_block[s_idx] = srd;
                    }
                    Step::SetUniformVec3F32 { .. } => {
                        let sr = StepRunnerSetUniformVec3F32::default();
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
                    Step::Label { .. } => {}
                    Step::Nop => {}
                }
                pos.inc_y();
                s_idx += 1;
            }
            self.step_runner_data
                .insert(String::from("start"), srd_block); // :TODO: "start" is not the right name
        }
        Ok(())
    }
    pub fn run_update(&mut self, gl: &Gl) -> Result<()> {
        // update time
        let now = std::time::Instant::now();
        let t = now - self.start_time;

        self.time = t.as_secs_f64();

        if self.time > 2048.0 {
            self.start_time = now;
        }

        if let Some(start_step) = self.flow.steps().iter().find(|(s, _gp)| {
            if let Step::Label { name, .. } = s {
                name == "start"
            } else {
                false
            }
        }) {
            let (_s, pos) = start_step;
            // eprintln!("Starting at {pos:?}");
            let mut pos = pos.clone();
            let mut s_idx = 0;
            let srd_block = self
                .step_runner_data
                .get("start")
                .ok_or(eyre!("Data for block `start` not found"))?;

            while let Some(step) = self.flow.get_step_at(&pos) {
                // eprintln!("Update {s_idx} {step:?}");
                match step {
                    Step::Program { .. } => {
                        let sr = StepRunnerProgram::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, srd);
                    }
                    Step::SetUniformF32 { .. } => {
                        let sr = StepRunnerSetUniformF32::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, &self, step, srd);
                    }
                    Step::SetUniformF64 { .. } => {
                        let sr = StepRunnerSetUniformF64::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, &self, step, srd);
                    }
                    Step::SetUniformVec3F32 { .. } => {
                        let sr = StepRunnerSetUniformVec3F32::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, &self, step, srd);
                    }
                    Step::FullscreenQuad => {
                        let sr = StepRunnerFullscreenQuad::default();

                        let srd = &srd_block[s_idx];
                        sr.run_render(gl, srd);
                    }
                    Step::Label { .. } => {}
                    Step::Nop => {}
                }
                pos.inc_y();
                s_idx += 1;
            }
        }
        Ok(())
    }
}
