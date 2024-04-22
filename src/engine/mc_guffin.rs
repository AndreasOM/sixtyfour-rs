use crate::engine::ShaderSource;
use super::gl::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::c_void;
use core::ffi::CStr;
use std::collections::HashMap;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct McGuffin {
    gl: Gl,

    vertex_array_id: u32,
    vertex_buffer_id: u32,
    program: u32,
    properties: HashMap<String, f32>,
    shader_sources: HashMap<String, ShaderSource>,

    test: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StoredMcGuffin {
    test: String,
    shader_sources: HashMap<String, ShaderSource>,
}

impl From<&McGuffin> for StoredMcGuffin {
    fn from(mc:&McGuffin) -> Self {
        Self {
            test: mc.test.clone(),
            shader_sources: mc.shader_sources.clone(),
        }
    }
}

impl From<StoredMcGuffin> for McGuffin {
    fn from(smc:StoredMcGuffin) -> Self {
        let s = Self {
            test: smc.test,
            shader_sources: smc.shader_sources,
            ..Default::default()
        };
        dbg!(&s.shader_sources);
        s
    }
}

unsafe impl Send for McGuffin {}

//static glRects: extern "system" fn(i16, i16, i16, i16) -> c_void = GlFunctionPointer::null().into();

impl McGuffin {
    fn load_function(
        get_proc_address: &dyn Fn(&CStr) -> *const c_void,
        name: &CStr,
    ) -> Result<*const c_void> {
        let addr = get_proc_address(name);
        if addr == core::ptr::null() {
            Err(eyre!("Failed loading {name:?}").into())
        } else {
            Ok(addr)
        }
    }
    pub fn get_shader_source(&self, name: &str ) -> Option<&ShaderSource> {
        self.shader_sources.get( name )
    }
    pub fn get_mut_shader_source(&mut self, name: &str ) -> Option<&mut ShaderSource> {
        self.shader_sources.get_mut( name )
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
    fn compile_shader(&mut self, shader_type: GLenum, shader_source: &str) -> Result<GLuint> {
        // :TODO: verify shader type
        let shader_source = CString::new(shader_source)?;
        let shader = self.gl.glCreateShader(shader_type);

        self.gl.glShaderSource(
            shader,
            1,
            &shader_source.as_ptr() as *const *const _,
            core::ptr::null(),
        );
        self.gl.glCompileShader(shader);

        let mut status: GLint = GL_FALSE as GLint;
        self.gl
            .glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed compiling shader");
            let mut len = 0;
            self.gl.glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut len);
            dbg!(len);
            let mut buf = Vec::with_capacity(len as usize);
            unsafe { buf.set_len((len as usize) - 1) };
            let mut len = len as u32;
            self.gl
                .glGetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);
            let log = String::from_utf8_lossy(&buf);
            dbg!(log);
            return Err(eyre!("Failed compiling shader!").into());
        }
        self.check_gl_error(std::line!());

        Ok(shader)
    }

    fn link_program(&mut self, vertex_shader: u32, fragment_shader: u32) -> Result<u32> {
        let program = self.gl.glCreateProgram();
        self.gl.glAttachShader(program, vertex_shader);
        self.gl.glAttachShader(program, fragment_shader);
        self.gl.glLinkProgram(program);

        let mut status: GLint = GL_FALSE as GLint;
        self.gl.glGetProgramiv(program, GL_LINK_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed linking program");
            let mut len = 0;
            self.gl
                .glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut len);
            dbg!(len);
            let mut buf = Vec::with_capacity(len as usize);
            unsafe { buf.set_len((len as usize) - 1) };
            let mut len = len as u32;
            self.gl
                .glGetProgramInfoLog(program, len, &mut len, buf.as_mut_ptr() as *mut _);
            let log = String::from_utf8_lossy(&buf);
            dbg!(log);
            return Err(eyre!("Failed linking program").into());
        }
        self.check_gl_error(std::line!());

        self.program = program;
        Ok(program)
    }
    fn add_shader_source(&mut self, name: &str, shader_type: GLenum, source: &str) {
        let ss = ShaderSource::new( shader_type, source.into() );
        self.shader_sources.insert(name.into(), ss);
    }
    fn load_shader_sources(&mut self) -> Result<()> {
        let mut loaded = false;
        if let Some( ss ) = self.get_mut_shader_source( "vertex") {
            if let Some( sp ) = ss.save_path() {
                let _ = ss.reload();
                loaded = true;
            }
        }
        if !loaded {
            self.add_shader_source(
                "vertex",
                GL_VERTEX_SHADER,
                &String::from_utf8_lossy(include_bytes!("../../assets/default.vert.glsl")),
            );
        }

        let mut loaded = false;
        if let Some( ss ) = self.get_mut_shader_source( "fragment") {
            if let Some( sp ) = ss.save_path() {
                let _ = ss.reload();
                loaded = true;
            }
        }
        if !loaded {
            eprintln!("Initialising fragment shader with baked in default");
            self.add_shader_source(
                "fragment",
                GL_FRAGMENT_SHADER,
                &String::from_utf8_lossy(include_bytes!("../../assets/default.frag.glsl")),
            );
        }

        Ok(())
    }
    pub fn setup(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        eprintln!("Test is {}", &self.test );
        self.test = String::from("42");
        eprintln!("Test is {}", &self.test );
        self.load_shader_sources()?;

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
        self.check_gl_error(std::line!());

        dbg!(&vertex_buffer_id);
        self.gl.bind_buffer(GL_ARRAY_BUFFER, vertex_buffer_id);
        //self.call_gl_bind_buffer( GL_ARRAY_BUFFER, 0 );
        self.check_gl_error(std::line!());
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

        self.rebuild_program()?;

        Ok(())
        //Err( eyre!("test") )
    }

    pub fn rebuild_program(&mut self) -> Result<()> {
        let vertex_shader_source = String::from(self.get_shader_source_source("vertex"));
        let fragment_shader_source = String::from(self.get_shader_source_source("fragment"));

        self.mark_shader_source_clean("vertex");
        self.mark_shader_source_clean("fragment");

        let vertex_shader = self.compile_shader(GL_VERTEX_SHADER, &vertex_shader_source)?;
        let fragment_shader = self.compile_shader(GL_FRAGMENT_SHADER, &fragment_shader_source)?;

        self.program = self.link_program(vertex_shader, fragment_shader)?;
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
        self.check_gl_error(std::line!());
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

        self.gl.glUseProgram(self.program);
        self.check_gl_error(std::line!());

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
        for (k, v) in self.properties.iter() {
            let n = CString::new(String::from(k))?;
            let l = self.gl.glGetUniformLocation(self.program, n.as_ptr());
            if l == -1 {
                eprintln!("Uniform {k} not found");
            } else {
                self.gl.glProgramUniform1f(self.program, l, *v);
            }
        }

        self.gl.draw_arrays(GL_TRIANGLE_STRIP, 0, 4);
        //self.call_gl_draw_arrays(GL_TRIANGLE_STRIP, 0, 10);

        //self.call_gl_rects( -1, -1, 1, 1 );
        self.check_gl_error(std::line!());
        Ok(())
    }

    pub fn paint(&mut self, gl: &eframe::glow::Context) {
        let _ = self.update();
    }

    fn check_gl_error(&self, line: u32) {
        //let error = self.gl.get_error(); //self.call_gl_get_error();
        let error = unsafe { self.gl.glGetError() }; //self.call_gl_get_error();
        match error {
            0 => {}
            0x500 => {
                eprintln!("GL_INVALID_ENUM - Line {line}");
            }
            0x0502 => {
                eprintln!("GL_INVALID_OPERATION - Line {line}");
            }
            e => {
                eprintln!("0x{e:04x?}");
            }
        }
    }

    pub fn set_property(&mut self, name: &str, value: f32) {
        self.properties.insert(name.into(), value);
    }
}
