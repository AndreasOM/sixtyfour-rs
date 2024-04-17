use crate::RotatingTriangle;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::c_int;
use core::ffi::c_short;
use core::ffi::c_uchar;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::CStr;
use core::mem::transmute;

use super::gl::*;

#[derive(Debug, Default)]
pub struct McGuffin {
    gl: Gl,

    vertex_array_id: u32,
    vertex_buffer_id: u32,

    rotating_triangle: Option<RotatingTriangle>,
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
        self.gl.gen_vertex_arrays(1, &mut vertex_array_id);
        dbg!(&vertex_array_id);
        self.gl.bind_vertex_array(vertex_array_id);

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

        /*
                    let (vertex_shader_source, fragment_shader_source) = (
                        r#"
                            layout(location=0)in vec2 v;
                            layout(location=0)out vec2 p;
                            void main() {
                                gl_Position = vec4( v, 0.0, 1.0);
                                p = v;
                            }
                        "#,
                        r#"
                            precision mediump float;
                            out vec4 out_color;
                            layout(location=0)in vec2 p;
                            void main() {
                                out_color = vec4( sin( p.x*11 ), sin( p.y*15.0 ), 1., 1.0 );
                            }
                        "#,
                    );

        */
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

        self.gl.bind_vertex_array(self.vertex_array_id);
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
        self.gl.draw_arrays(GL_TRIANGLE_STRIP, 0, 4);
        //self.call_gl_draw_arrays(GL_TRIANGLE_STRIP, 0, 10);

        //self.call_gl_rects( -1, -1, 1, 1 );
        self.check_gl_error(std::line!());
        Ok(())
    }

    pub fn paint(&mut self, gl: &eframe::glow::Context) {
        if let Some(rt) = &self.rotating_triangle {
            rt.paint(gl, 0.0);
        }
        let _ = self.update();
    }

    pub fn setup_rotating_triangle(&mut self, gl: &eframe::glow::Context) {
        let rt = RotatingTriangle::new(gl);
        self.rotating_triangle = Some(rt);
    }

    fn check_gl_error(&self, line: u32) {
        let error = self.gl.get_error(); //self.call_gl_get_error();
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
