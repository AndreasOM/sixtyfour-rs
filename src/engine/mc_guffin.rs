use crate::engine::ResourceLogManager;
use super::gl::*;
use crate::engine::FlowVm;
use crate::project::Project;
use crate::project::PropertyValue;
use crate::project::ResourceId;
use color_eyre::Result;
use core::ffi::c_void;
use core::ffi::CStr;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct McGuffin {
    gl: Gl,

    //pipeline: Pipeline,
    properties_f32: HashMap<String, f32>,
    properties_vec2_f32: HashMap<String, [f32; 2]>,
    properties_vec3_f32: HashMap<String, [f32; 3]>,
    properties_vec3_f32_size4: HashMap<String, [f32; 3 * 4]>,
    // shader_sources: HashMap<String, ShaderSource>,

    project: Project,
    project_version: u32,

    last_paint_duration: std::time::Duration,
    flow_vm: FlowVm,

    resource_log_manager: ResourceLogManager,
}

// :TODO: remove
unsafe impl Send for McGuffin {}

//static glRects: extern "system" fn(i16, i16, i16, i16) -> c_void = GlFunctionPointer::null().into();

//const EMPTY_VEC_STRING: &Vec<String> = &vec![];
const MILLIS_PER_SEC: u64 = 1_000;
const NANOS_PER_MILLI: u32 = 1_000_000;

impl McGuffin {
    pub fn last_paint_duration_in_ms(&self) -> f32 {
        (self.last_paint_duration.as_secs() as f32) * (MILLIS_PER_SEC as f32)
            + (self.last_paint_duration.as_nanos() as f32) / (NANOS_PER_MILLI as f32)
        //self.last_paint_duration.as_millis_f32()
    }
    /*
    pub fn uniform_manager(&self) -> &UniformManager {
        &self.pipeline.uniform_manager()
    }
    */
    pub fn get_resource_log(&self, resource_id: &ResourceId) -> Cow<'_, Vec<String>> {
        /*
        for (_n, ss) in self.shader_sources.iter() {
            if Some(resource_id) == ss.resource_id() {
                return Cow::Borrowed(ss.compile_log());
            }
        }
        */
        if let Some( l ) = self.resource_log_manager.get( resource_id ) {
            return Cow::Borrowed( l );
        }

        let not_found = vec![format!("Resource '{resource_id}' not found")];
        Cow::Owned(not_found)
    }
    pub fn setup(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        // load the gl functions we need
        // glRects

        self.gl.load_all(get_proc_address)?;

        // self.flow_vm.run_setup(&self.gl, &self.project)?;

        // create the program (vertex + fragment)
        // self.pipeline.setup(&mut self.gl)?;

        Ok(())
    }
    pub fn update(&mut self) -> Result<()> {
        /*
        self.pipeline.bind(&mut self.gl)?;
        self.gl.check_gl_error(std::file!(), std::line!());

        self.gl.check_gl_error(std::file!(), std::line!());
        for (k, v) in self.properties_f32.iter() {
            let _ = self.pipeline.set_property(&mut self.gl, k, *v);
        }
        for (k, v) in self.properties_vec2_f32.iter() {
            let _ = self.pipeline.set_property_vec2_f32(&mut self.gl, k, v);
        }
        for (k, v) in self.properties_vec3_f32.iter() {
            let _ = self.pipeline.set_property_vec3_f32(&mut self.gl, k, v);
        }
        for (k, v) in self.properties_vec3_f32_size4.iter() {
            let _ = self
                .pipeline
                .set_property_vec3_f32_size4(&mut self.gl, k, v);
        }
        self.gl.check_gl_error(std::file!(), std::line!());
        */
        self.flow_vm.run_update(&self.gl)?;
        self.gl.check_gl_error(std::file!(), std::line!());
        Ok(())
    }

    pub fn paint(&mut self, _gl: &eframe::glow::Context) {
        let paint_start = std::time::Instant::now();
        let _ = self.update();
        self.gl.glFinish();
        let paint_end = std::time::Instant::now();
        let paint_duration = paint_end - paint_start;
        // eprintln!("{paint_duration:?}");
        self.last_paint_duration = paint_duration;
    }

    fn set_property_f32(&mut self, name: &str, value: f32) {
        self.properties_f32.insert(name.into(), value);
    }
    fn set_property_vec2_f32(&mut self, name: &str, values: &[f32; 2]) {
        self.properties_vec2_f32.insert(name.into(), *values);
    }
    fn set_property_vec3_f32(&mut self, name: &str, values: &[f32; 3]) {
        self.properties_vec3_f32.insert(name.into(), *values);
    }
    fn set_property_vec3_f32_size4(&mut self, name: &str, values: &[f32; 3 * 4]) {
        self.properties_vec3_f32_size4.insert(name.into(), *values);
    }

    pub fn update_from_project(&mut self, project: &Project) {
        if self.project_version != project.version() {
            eprintln!("Project changed {}", project.version());
            self.project = (*project).clone();
            let _todo = self.flow_vm.load(self.project.flow());
            let _todo = self.flow_vm.run_setup(&self.gl, project, &mut self.resource_log_manager);
            self.project_version = project.version();
        }
        /*
                match self.rebuild_program() {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Rebuild Program failed: {e:?}");
                    }
                }
        */
        project.with_property_manager(|pm| {
            for (k, p) in pm.entries().iter() {
                // :FUTURE: self.set_property( k, p );
                match p.value() {
                    PropertyValue::F32 { value, .. } => self.set_property_f32(k, *value),
                    PropertyValue::Vec2F32 { values } => self.set_property_vec2_f32(k, &values),
                    PropertyValue::Vec3F32 { values } => self.set_property_vec3_f32(k, &values),
                    PropertyValue::Vec3F32Size4 { values } => {
                        self.set_property_vec3_f32_size4(k, &values)
                    }
                    v => {
                        eprintln!("Update for PropertyValue {v:?} not implemented");
                    }
                }
            }
        });
    }

    pub fn set_time(&mut self, time: f32) {
        self.set_property_f32("fTime", time);
    }
}
