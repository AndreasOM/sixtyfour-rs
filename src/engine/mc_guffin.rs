use super::gl::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::c_void;
use core::ffi::CStr;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct McGuffin {
    gl: Gl,

    vertex_array_id: u32,
    vertex_buffer_id: u32,
    program: u32,
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

        let (vertex_shader_source, fragment_shader_source) = (
            r#"#version 410
                            

                            layout(location=0)in vec2 v;
                            layout(location=0)out vec2 p;
                            void main() {
                                gl_Position = vec4( v, 0.0, 1.0);
                                p = v;
                            }
                        "#,
            r#"#version 410
                            

                            precision mediump float;
                            out vec4 out_color;
                            layout(location=0)in vec2 p;
                            void main() {
                                out_color = vec4( sin( p.x*11 ), sin( p.y*15.0 ), sin( ( p.x + p.y ) * 3.0 ), 1.0 );
                            }
                        "#,
        );
        //let vertex_shader_source = "";
        let vertex_shader_source = CString::new(vertex_shader_source)?;
        let vertex_shader = self.gl.glCreateShader(GL_VERTEX_SHADER);

        self.gl.glShaderSource(
            vertex_shader,
            1,
            &vertex_shader_source.as_ptr() as *const *const _,
            core::ptr::null(),
        );
        self.gl.glCompileShader(vertex_shader);

        let mut status: GLint = GL_FALSE as GLint;
        self.gl
            .glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed compiling vertex shader");
            let mut len = 0;
            self.gl
                .glGetShaderiv(vertex_shader, GL_INFO_LOG_LENGTH, &mut len);
            dbg!(len);
            todo!();
        }
        self.check_gl_error(std::line!());

        let fragment_shader_source = CString::new(fragment_shader_source)?;
        let fragment_shader = self.gl.glCreateShader(GL_FRAGMENT_SHADER);

        self.gl.glShaderSource(
            fragment_shader,
            1,
            &fragment_shader_source.as_ptr() as *const *const _,
            core::ptr::null(),
        );
        self.gl.glCompileShader(fragment_shader);

        let mut status: GLint = GL_FALSE as GLint;
        self.gl
            .glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed compiling fragment shader");
            todo!();
        }
        self.check_gl_error(std::line!());

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
            GL_DYNAMIC_DRAW,
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
}
