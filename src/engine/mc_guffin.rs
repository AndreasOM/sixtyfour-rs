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
}

#[derive(Debug, Default)]
struct ShaderSource {
    pub shader_type: GLenum,
    pub source: String,
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
    pub fn get_shader_source(&self, name: &str) -> &str {
        if let Some(ss) = self.shader_sources.get(name) {
            &ss.source
        } else {
            "shader does not exist"
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
    fn add_shader_source(&mut self, name: &str, shader_type: GLenum, source: &str) {
        let ss = ShaderSource {
            shader_type,
            source: source.into(),
        };

        self.shader_sources.insert(name.into(), ss);
    }
    fn load_shader_sources(&mut self) -> Result<()> {
        self.add_shader_source(
            "vertex",
            GL_VERTEX_SHADER,
            r#"#version 410
                            

                            layout(location=0)in vec2 v;
                            layout(location=0)out vec2 p;
                            void main() {
                                gl_Position = vec4( v, 0.0, 1.0);
                                p = v;
                            }
                        "#,
        );
        self.add_shader_source( "fragment", GL_VERTEX_SHADER,
r#"#version 410
    uniform float fTime;
    uniform float speed;
    uniform float scale_red_x;
    uniform float scale_green_y;

    precision mediump float;
    out vec4 out_color;
    layout(location=0)in vec2 p;

    float rand(float n){
        return fract(sin(n) * 43758.5453123);
    }
    float rand(vec2 n) { 
        return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
    }

    float noise(float p){
        float fl = floor(p);
        float fc = fract(p);
        return mix(rand(fl), rand(fl + 1.0), fc);
    }
    float noise(vec2 n) {
        const vec2 d = vec2(0.0, 1.0);
        vec2 b = floor(n), f = smoothstep(vec2(0.0), vec2(1.0), fract(n));
        return mix(mix(rand(b), rand(b + d.yx), f.x), mix(rand(b + d.xy), rand(b + d.yy), f.x), f.y);
    }
    float n1( float x ) {
        #define hash(v) fract(sin(100.0*v)*4375.5453)
        float f = fract(x);
        float p = floor(x);

        f = f*f*(3.0-2.0*f);

        return mix(hash(f),hash(p),f);
    }
    
    void main() {
        float time = speed*fTime;
        vec2 n2 = noise2( p + vec2( time*0.03, sin(time*0.032) ));
        float rn = n2.x*5.0;//noise( p.x ); //n1(p.x);
        float gn = n2.y*3.0;//n1(p.y);
        out_color = vec4( sin( time+rn+p.x*scale_red_x ), sin( gn+p.y*scale_green_y ), sin( ( p.x + p.y ) * 3.0 ), 1.0 );
        //out_color = vec4( rn, rn, rn, 1.0 );
    }
"#
        );

        Ok(())
    }
    pub fn setup(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
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

        let vertex_shader_source = String::from(self.get_shader_source("vertex"));
        let fragment_shader_source = String::from(self.get_shader_source("fragment"));

        let vertex_shader = self.compile_shader(GL_VERTEX_SHADER, &vertex_shader_source)?;
        let fragment_shader = self.compile_shader(GL_FRAGMENT_SHADER, &fragment_shader_source)?;

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
            todo!();
        }
        self.check_gl_error(std::line!());

        self.program = program;

        Ok(())
        //Err( eyre!("test") )
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
