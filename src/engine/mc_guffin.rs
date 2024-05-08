use super::gl::*;
use crate::engine::Pipeline;
use crate::engine::ShaderSource;
use crate::engine::UniformManager;
use crate::project::Project;
use crate::project::PropertyValue;
use crate::project::Resource;
use crate::project::ResourceId;
use crate::project::ShaderType;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::c_void;
use core::ffi::CStr;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct McGuffin {
    gl: Gl,

    vertex_array_id: u32,
    vertex_buffer_id: u32,
    pipeline: Pipeline,
    properties_f32: HashMap<String, f32>,
    properties_vec2_f32: HashMap<String, [f32; 2]>,
    properties_vec3_f32: HashMap<String, [f32; 3]>,
    properties_vec3_f32_size4: HashMap<String, [f32; 3 * 4]>,
    shader_sources: HashMap<String, ShaderSource>,

    project: Project,
}

// :TODO: remove
unsafe impl Send for McGuffin {}

//static glRects: extern "system" fn(i16, i16, i16, i16) -> c_void = GlFunctionPointer::null().into();

//const EMPTY_VEC_STRING: &Vec<String> = &vec![];

impl McGuffin {
    pub fn uniform_manager(&self) -> &UniformManager {
        &self.pipeline.uniform_manager()
    }
    pub fn get_resource_log(&self, resource_id: &ResourceId) -> Cow<'_, Vec<String>> {
        for (_n, ss) in self.shader_sources.iter() {
            if Some(resource_id) == ss.resource_id() {
                return Cow::Borrowed(ss.compile_log());
            }
        }
        let not_found = vec![format!("Resource '{resource_id}' not found")];

        Cow::Owned(not_found)
    }
    fn get_shader_source(&self, name: &str) -> Option<&ShaderSource> {
        self.shader_sources.get(name)
    }
    fn get_mut_shader_source(&mut self, name: &str) -> Option<&mut ShaderSource> {
        self.shader_sources.get_mut(name)
    }
    pub fn is_shader_source_dirty(&self, name: &str) -> bool {
        if let Some(ss) = self.shader_sources.get(name) {
            ss.dirty()
        } else {
            false
        }
    }
    pub fn get_shader_source_source(&self, name: &str) -> &str {
        if let Some(ss) = self.shader_sources.get(name) {
            ss.source()
        } else {
            "shader does not exist"
        }
    }
    pub fn replace_shader_source(&mut self, name: &str, source: String) {
        if let Some(ss) = self.shader_sources.get_mut(name) {
            ss.update_source(source);
        } else {
            eprintln!("ShaderSource {name} not found!");
        }
    }
    pub fn mark_shader_source_clean(&mut self, name: &str) {
        if let Some(ss) = self.shader_sources.get_mut(name) {
            ss.mark_clean();
        } else {
            eprintln!("ShaderSource {name} not found!");
        }
    }
    fn add_shader_source(&mut self, name: &str, shader_type: GLenum, source: &str) {
        let ss = ShaderSource::new(shader_type, source.into());
        self.shader_sources.insert(name.into(), ss);
    }

    pub fn setup(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        // load the gl functions we need
        // glRects

        self.gl.load_all(get_proc_address)?;
        // create the program (vertex + fragment)

        // prepare the buffers
        let mut vertex_array_id = 0;
        self.gl.glGenVertexArrays(1, &mut vertex_array_id);
        dbg!(&vertex_array_id);
        self.gl.glBindVertexArray(vertex_array_id);

        let mut vertex_buffer_id = 0;
        self.gl.gen_buffers(1, &mut vertex_buffer_id);
        self.gl.check_gl_error(std::file!(), std::line!());

        dbg!(&vertex_buffer_id);
        self.gl.bind_buffer(GL_ARRAY_BUFFER, vertex_buffer_id);
        //self.call_gl_bind_buffer( GL_ARRAY_BUFFER, 0 );
        self.gl.check_gl_error(std::file!(), std::line!());
        /*
                let data = [
                    0.5f32, 1.0,
                    -1.0, -1.0,
                    1.0, -1.0,
                    1.0, -1.0,
                ];
        */
        self.do_data();

        self.vertex_array_id = vertex_array_id;
        self.vertex_buffer_id = vertex_buffer_id;

        self.pipeline.setup(&mut self.gl)?;

        Ok(())
    }

    pub fn rebuild_program(&mut self) -> Result<()> {
        // update shader sources from project
        // and check for changes

        let mut program_changed = false;
        for (_id, r) in self.project.resource_manager.resources() {
            match r {
                Resource::Program(rp) => {
                    for s in rp.shaders() {
                        let resource_id = s.resource_id();
                        if let Some(r) = self.project.resource_manager.get(resource_id) {
                            if let Resource::Text(rt) = r {
                                if !rt.text().is_empty() {
                                    //println!("{:?} {rt:?}", s.shader_type());
                                    match s.shader_type() {
                                        ShaderType::Fragment => {
                                            if let Some(fss) =
                                                self.shader_sources.get_mut("fragment")
                                            {
                                                if rt.version() > fss.last_project_version {
                                                    println!("Fragment shader changed");

                                                    fss.last_project_version = rt.version();
                                                    fss.update_source(rt.text().to_owned());
                                                    fss.set_resource_id(resource_id);
                                                    program_changed = true;
                                                }
                                            } else {
                                                eprintln!("No fragment shader");
                                                let mut s = ShaderSource::new(
                                                    GL_FRAGMENT_SHADER,
                                                    rt.text().to_string(),
                                                );
                                                s.last_project_version = rt.version();
                                                s.set_resource_id(resource_id);
                                                self.shader_sources
                                                    .insert(String::from("fragment"), s);
                                                program_changed = true;
                                            }
                                        }
                                        ShaderType::Vertex => {
                                            if let Some(fss) = self.shader_sources.get_mut("vertex")
                                            {
                                                if rt.version() > fss.last_project_version {
                                                    println!("Vertex shader changed");

                                                    fss.last_project_version = rt.version();
                                                    fss.update_source(rt.text().to_owned());
                                                    fss.set_resource_id(resource_id);
                                                    program_changed = true;
                                                }
                                            } else {
                                                eprintln!("No vertex shader");
                                                let mut s = ShaderSource::new(
                                                    GL_VERTEX_SHADER,
                                                    rt.text().to_string(),
                                                );
                                                s.last_project_version = rt.version();
                                                s.set_resource_id(resource_id);
                                                self.shader_sources
                                                    .insert(String::from("vertex"), s);
                                                program_changed = true;
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    //program_changed = true;
                    break;
                }
                _ => {}
            }
        }

        if program_changed {
            eprintln!("Program Changed: {program_changed:?}");
            self.pipeline
                .rebuild(&mut self.gl, &mut self.shader_sources)
                .map_err(|e| {
                    eprintln!("Pipeline rebuild failed: {e:?}");
                    e
                })?;
            for ss in self.shader_sources.values_mut() {
                ss.mark_clean();
            }
        }
        Ok(())
    }

    fn do_data(&self) {
        let data: &mut [f32] = &mut [
            1.0, -1.0, // top right -> bottom right?
            1.0, 1.0, // top right -> top right?
            -1.0, -1.0, // top left -> bottom left?
            -1.0, 1.0, // top right -> top left?
        ];
        /*
        let mut rng = rand::thread_rng();

        for i in 0..=5 {
            data[ i ] = rng.gen_range(-1.0..1.0);
        }
        */
        /*
        for f in &mut *data {
            //let r = rng.next_u32() as f32;
            let r: f32 = rng.gen_range(-1.0..1.0);

            *f = r / f32::MAX;
        }
        */

        //let data = [-1.0, -1.0, -1.0, 0.5, 0.5, -1.0, 0.5, 0.5];
        //let size = core::mem::size_of_val(&data);
        let size = 4 * data.len();
        dbg!(&size);
        //dbg!(data.as_ptr() as *const _);
        self.gl.buffer_data(
            GL_ARRAY_BUFFER,
            size as isize,
            data.as_ptr() as *const _,
            GL_STATIC_DRAW,
        );
        self.gl.check_gl_error(std::file!(), std::line!());
    }

    pub fn update(&mut self) -> Result<()> {
        // bind the program

        // pass in uniforms
        // e.g. current time

        // render something
        // -> e.g. a fullscreen (or rather full viewport) rectangle

        //glRects( -1, -1, 1, 1);
        //self.call_gl_rects(-1, -1, 1, 1);

        // gl::DrawArrays(gl::TRIANGLES, 0, 6i32);

        self.pipeline.bind(&mut self.gl)?;
        //self.gl.glUseProgram(self.program);
        self.gl.check_gl_error(std::file!(), std::line!());

        self.gl.glBindVertexArray(self.vertex_array_id);
        //dbg!(self.vertex_array_id);
        self.gl.bind_buffer(GL_ARRAY_BUFFER, self.vertex_buffer_id);
        //dbg!(self.vertex_buffer_id);
        /*
                let data = [0.0;8];
                let size = core::mem::size_of_val(&data);
                dbg!(&size);
                dbg!(data.as_ptr() as *const _);
                self.call_gl_buffer_data(
                    GL_ARRAY_BUFFER,
                    size as isize,
                    data.as_ptr() as *const _,
                    GL_STATIC_DRAW,
                );
                self.check_gl_error(std::line!());
        */
        self.gl.enable_vertex_attrib_array(0); // 0 == pos

        self.gl
            .vertex_attrib_pointer(0, 2, GL_FLOAT, GL_FALSE as u8, 0, core::ptr::null());

        //self.do_data();
        //self.call_gl_disable( GL_CULL_FACE );
        //unsafe{ self.gl.glDisable( GL_CULL_FACE ); }

        // set uniforms
        // glGetUniformLocation
        // glProgramUniform1f
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

        self.gl.draw_arrays(GL_TRIANGLE_STRIP, 0, 4);
        //self.call_gl_draw_arrays(GL_TRIANGLE_STRIP, 0, 10);

        //self.call_gl_rects( -1, -1, 1, 1 );
        self.gl.check_gl_error(std::file!(), std::line!());
        Ok(())
    }

    pub fn paint(&mut self, _gl: &eframe::glow::Context) {
        let _ = self.update();
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
        self.project = (*project).clone();
        match self.rebuild_program() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Rebuild Program failed: {e:?}");
            }
        }
        for (k, p) in project.property_manager.entries().iter() {
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
    }

    pub fn set_time(&mut self, time: f32) {
        self.set_property_f32("fTime", time);
    }
}
