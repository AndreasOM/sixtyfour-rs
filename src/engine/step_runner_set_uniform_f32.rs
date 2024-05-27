use crate::engine::gl::GLint;
use crate::engine::gl::Gl;
use crate::engine::StepRunnerData;
use crate::project::Step;
use core::any::Any;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct StepRunnerSetUniformF32 {}

impl StepRunnerSetUniformF32 {
    pub fn create_data(&self) -> Option<Box<dyn StepRunnerData>> {
        let d = StepRunnerDataSetUniformF32::default();
        Some(Box::new(d))
    }
    pub fn run_setup(&self, gl: &Gl, step: &Step, data: &mut Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data
                .as_any_mut()
                .downcast_mut::<StepRunnerDataSetUniformF32>()
            {
                Some(data) => {
                    if let Step::SetUniformF32 { name, .. } = step {
                        let n =
                            CString::new(String::from(name)).expect("can convert name to CString"); // what the elf?
                        let program = 4; // :HACK:
                        let l = gl.glGetUniformLocation(program, n.as_ptr());
                        eprintln!("Location for {name} -> {l}");
                        data.location = l;
                    }
                }
                None => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    }
    pub fn run_teardown(&self, _data: &mut Option<Box<dyn StepRunnerData>>) {}
    pub fn run_render(&self, gl: &Gl, step: &Step, data: &Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data.as_any().downcast_ref::<StepRunnerDataSetUniformF32>() {
                Some(data) => {
                    if let Step::SetUniformF32 { value, .. } = step {
                        if data.location >= 0 {
                            let program = 4; // :HACK:
                            let value = value.parse::<f32>().unwrap_or_default();
                            gl.glProgramUniform1f(program, data.location, value);
                        }
                    }
                }
                None => {
                    unimplemented!();
                }
            }
        } else {
            unimplemented!();
        }
    }
}

#[derive(Debug, Default)]
struct StepRunnerDataSetUniformF32 {
    location: GLint,
}

impl StepRunnerData for StepRunnerDataSetUniformF32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
