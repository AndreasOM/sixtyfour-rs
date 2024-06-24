use crate::engine::gl::GLint;
use crate::engine::gl::Gl;
use crate::engine::gl::GL_CURRENT_PROGRAM;
use crate::engine::FlowVm;
use crate::engine::StepRunnerData;
use crate::project::Step;
use core::any::Any;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct StepRunnerSetUniformVec3F32 {}

impl StepRunnerSetUniformVec3F32 {
    pub fn create_data(&self) -> Option<Box<dyn StepRunnerData>> {
        let d = StepRunnerDataSetUniformVec3F32::default();
        Some(Box::new(d))
    }
    pub fn run_setup(&self, gl: &Gl, step: &Step, data: &mut Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data
                .as_any_mut()
                .downcast_mut::<StepRunnerDataSetUniformVec3F32>()
            {
                Some(data) => {
                    if let Step::SetUniformVec3F32 { name, .. } = step {
                        let n =
                            CString::new(String::from(name)).expect("can convert name to CString"); // what the elf?
                        let mut program: GLint = 0;
                        gl.glGetIntegerv(GL_CURRENT_PROGRAM, &mut program);
                        let l = gl.glGetUniformLocation(program as u32, n.as_ptr());
                        eprintln!("Location for {name} -> {l} in {program}");
                        if l < 0 {
                            //panic!("");
                        }
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
    pub fn run_render(
        &self,
        gl: &Gl,
        flow_vm: &FlowVm,
        step: &Step,
        data: &Option<Box<dyn StepRunnerData>>,
    ) {
        if let Some(data) = data {
            match data
                .as_any()
                .downcast_ref::<StepRunnerDataSetUniformVec3F32>()
            {
                Some(data) => {
                    if let Step::SetUniformVec3F32 { values, name, .. } = step {
                        if data.location >= 0 {
                            let mut program: GLint = 0;
                            gl.glGetIntegerv(GL_CURRENT_PROGRAM, &mut program);
                            let values: Vec<_> =
                                values.iter()
                                    .map(|value|{
                                        value.parse::<f32>()
                                        .unwrap_or_else(|_| match value.as_ref() {
                                            "${TIME}" => (flow_vm.time().rem_euclid(2048.0)) as f32, // wrap to avoid precision issues, will _jerk_ after ~0.5h
                                            _ => 0.0,                                        
                                        })
                                    }).collect();
                            //if value != data.value {
                            //eprintln!("Value changed to {value} for {name}");
                            //data.value = value;
                            gl.glProgramUniform3fv(program as u32, data.location, 1, values.as_ptr());
                            gl.check_gl_error(std::file!(), std::line!());
                            //}
                        } else {
                            eprintln!("Data location for {name}[f32;3] is ZERO");
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
struct StepRunnerDataSetUniformVec3F32 {
    location: GLint,
    values: [f32; 3],
}

impl StepRunnerData for StepRunnerDataSetUniformVec3F32 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
