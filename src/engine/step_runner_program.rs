use crate::engine::gl::Gl;
use crate::engine::gl::GL_FRAGMENT_SHADER;
use crate::engine::gl::GL_VERTEX_SHADER;
use crate::engine::Pipeline;
use crate::engine::ShaderSource;
use crate::engine::StepRunnerData;
use crate::project::Project;
use crate::project::Resource;
use crate::project::ResourceId;
use crate::project::ShaderType;
use crate::project::Step;
use core::any::Any;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StepRunnerProgram {}

impl StepRunnerProgram {
    pub fn create_data(&self) -> Option<Box<dyn StepRunnerData>> {
        let d = StepRunnerDataProgram::default();
        Some(Box::new(d))
    }
    fn shader_source_from_resource(
        project: &Project,
        resource_id: &ResourceId,
        shader_type: ShaderType,
    ) -> Option<(String, ShaderSource)> {
        if let Some(r) = project.resource_manager.get(resource_id) {
            if let Resource::Text(rt) = r {
                if !rt.text().is_empty() {
                    //println!("{:?} {rt:?}", s.shader_type());
                    match shader_type {
                        ShaderType::Fragment => {
                            let mut s =
                                ShaderSource::new(GL_FRAGMENT_SHADER, rt.text().to_string());
                            s.last_project_version = rt.version();
                            s.set_resource_id(resource_id);
                            return Some((String::from("fragment"), s));
                        }
                        ShaderType::Vertex => {
                            let mut s = ShaderSource::new(GL_VERTEX_SHADER, rt.text().to_string());
                            s.last_project_version = rt.version();
                            s.set_resource_id(resource_id);
                            return Some((String::from("vertex"), s));
                        }
                    }
                }
            }
        }
        None
    }
    pub fn run_setup(
        &self,
        gl: &Gl,
        project: &Project,
        step: &Step,
        data: &mut Option<Box<dyn StepRunnerData>>,
    ) {
        eprintln!("Setting up program from step {step:?}");
        if let Step::Program { resource_id } = step {
            if let Some(data) = data {
                match data.as_any_mut().downcast_mut::<StepRunnerDataProgram>() {
                    Some(data) => {
                        let mut pipeline = Pipeline::default();
                        let _todo = pipeline.setup(gl);
                        // shader_sources: &mut HashMap<String, ShaderSource>,
                        let mut shader_sources = HashMap::default();

                        if let Some(r) = project.resource_manager.get(resource_id) {
                            match r {
                                Resource::Program(rp) => {
                                    for s in rp.shaders() {
                                        let resource_id = s.resource_id();
                                        if let Some((name, shader_source)) =
                                            Self::shader_source_from_resource(
                                                project,
                                                resource_id,
                                                s.shader_type(),
                                            )
                                        {
                                            shader_sources.insert(name, shader_source);
                                        } else {
                                            todo!();
                                        }
                                    }
                                }
                                _ => return,
                            }
                        }

                        // project.resource_manager.get(resource_id)

                        let _todo = pipeline.rebuild(gl, &mut shader_sources).map_err(|e| {
                            eprintln!("Failed rebuilding pipeline {e:?}");
                            e
                        });

                        gl.check_gl_error(std::file!(), std::line!());
                        data.pipeline = pipeline;
                    }
                    None => {
                        unimplemented!();
                    }
                }
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    }
    pub fn run_teardown(&self, _data: &mut Option<Box<dyn StepRunnerData>>) {}
    pub fn run_render(&self, gl: &Gl, data: &Option<Box<dyn StepRunnerData>>) {
        if let Some(data) = data {
            match data.as_any().downcast_ref::<StepRunnerDataProgram>() {
                Some(data) => {
                    let _todo = data.pipeline.bind(gl);
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
struct StepRunnerDataProgram {
    pipeline: Pipeline,
}

impl StepRunnerData for StepRunnerDataProgram {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
