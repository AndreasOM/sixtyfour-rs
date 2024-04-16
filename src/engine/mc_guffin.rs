use rand::Rng;

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

const GL_ARRAY_BUFFER: c_uint = 0x8892;
const GL_STATIC_DRAW: c_uint = 0x88E4;
const GL_DYNAMIC_DRAW: c_uint = 0x88E8;
const GL_FLOAT: c_uint = 0x1406;
const GL_FALSE: c_uint = 0x0000;
const GL_TRIANGLE_STRIP: c_uint = 0x0005;
const GL_CULL_FACE: c_uint = 0x0B44;

#[derive(Debug)]
struct GlFunctionPointer(*const c_void);
impl Default for GlFunctionPointer {
    fn default() -> Self {
        Self(core::ptr::null())
    }
}

impl GlFunctionPointer {
    const fn null() -> Self {
        Self(core::ptr::null())
    }
}

impl GlFunctionPointer {
    fn load(
        &mut self,
        get_proc_address: &dyn Fn(&CStr) -> *const c_void,
        name: &CStr,
    ) -> Result<()> {
        let addr = get_proc_address(name);
        if addr == core::ptr::null() {
            Err(eyre!("Failed loading {name:?}").into())
        } else {
            self.0 = addr;
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub struct McGuffin {
    // #[gl_call( (i16,i16,i16,i16) -> c_void )]
    gl_rects: GlFunctionPointer,
    //gl_rects: extern "system" fn(i16, i16, i16, i16) -> c_void,
    gl_get_error: GlFunctionPointer,
    gl_gen_vertex_arrays: GlFunctionPointer,
    gl_bind_vertex_array: GlFunctionPointer,
    gl_gen_buffers: GlFunctionPointer,
    gl_bind_buffer: GlFunctionPointer,
    gl_buffer_data: GlFunctionPointer,
    gl_enable_vertex_attrib_array: GlFunctionPointer,
    gl_vertex_attrib_pointer: GlFunctionPointer,
    gl_draw_arrays: GlFunctionPointer,
    gl_disable: GlFunctionPointer,

    vertex_array_id: u32,
    vertex_buffer_id: u32,

    rotating_triangle: Option<RotatingTriangle>,
}

unsafe impl Send for McGuffin {}

impl From<GlFunctionPointer> for extern "system" fn(i16, i16, i16, i16) -> c_void {
    fn from(p: GlFunctionPointer) -> Self {
        unsafe {
            transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(p.0)
        }
    }
}

//static glRects: extern "system" fn(i16, i16, i16, i16) -> c_void = GlFunctionPointer::null().into();

impl McGuffin {
    fn call_gl_rects(&self, x1: i16, y1: i16, x2: i16, y2: i16) {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(self.gl_rects.0);
            fn_p(x1, y1, x2, y2);
        };
    }

    fn call_gl_get_error(&self) -> c_uint {
        unsafe {
            let fn_p =
                transmute::<*const c_void, extern "system" fn() -> c_uint>(self.gl_get_error.0);
            fn_p()
        }
    }

    fn call_gl_gen_vertex_arrays(&self, n: c_int, arrays: *mut c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_int, *mut c_uint) -> c_void>(
                self.gl_gen_vertex_arrays.0,
            );
            fn_p(n, arrays)
        }
    }

    fn call_gl_bind_vertex_array(&self, array: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.gl_bind_vertex_array.0,
            );
            fn_p(array)
        }
    }

    fn call_gl_gen_buffers(&self, n: c_int, buffers: *mut c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_int, *mut c_uint) -> c_void>(
                self.gl_gen_buffers.0,
            );
            fn_p(n, buffers)
        }
    }

    fn call_gl_bind_buffer(&self, target: c_uint, buffer: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint, c_uint) -> c_void>(
                self.gl_bind_buffer.0,
            );
            fn_p(target, buffer)
        }
    }
    // -> GL_ARRAY_BUFFER

    fn call_gl_buffer_data(
        &self,
        target: c_uint,
        size: isize,
        data: *const c_void,
        usage: c_uint,
    ) -> c_void {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_uint, isize, *const c_void, c_uint) -> c_void,
            >(self.gl_buffer_data.0);
            fn_p(target, size, data, usage)
        }
    }
    // -> GL_ARRAY_BUFFER == 0x8892, GL_STATIC_DRAW == 0x88E4

    fn call_gl_enable_vertex_attrib_array(&self, index: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.gl_enable_vertex_attrib_array.0,
            );
            fn_p(index)
        }
    }

    fn call_gl_vertex_attrib_pointer(
        &self,
        index: c_uint,
        size: c_int,
        ttype: c_uint,
        normalized: c_uchar,
        stride: c_int,
        pointer: *const c_void,
    ) -> c_void {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_uint, c_int, c_uint, c_uchar, c_int, *const c_void) -> c_void,
            >(self.gl_vertex_attrib_pointer.0);
            fn_p(index, size, ttype, normalized, stride, pointer)
        }
    }

    fn call_gl_draw_arrays(&self, mode: c_uint, first: c_int, count: c_int) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint, c_int, c_int) -> c_void>(
                self.gl_draw_arrays.0,
            );
            fn_p(mode, first, count)
        }
    }
    // -> GL_TRIANGLE_STRIP == 0x0005

    fn call_gl_disable(&self, mode: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.gl_disable.0,
            );
            fn_p(mode)
        }
    }

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

        self.gl_rects.load(get_proc_address, c"glRects")?;
        self.gl_get_error.load(get_proc_address, c"glGetError")?;
        self.gl_gen_vertex_arrays
            .load(get_proc_address, c"glGenVertexArrays")?;
        self.gl_bind_vertex_array
            .load(get_proc_address, c"glBindVertexArray")?;
        self.gl_gen_buffers
            .load(get_proc_address, c"glGenBuffers")?;
        self.gl_bind_buffer
            .load(get_proc_address, c"glBindBuffer")?;
        self.gl_buffer_data
            .load(get_proc_address, c"glBufferData")?;
        self.gl_enable_vertex_attrib_array
            .load(get_proc_address, c"glEnableVertexAttribArray")?;
        self.gl_vertex_attrib_pointer
            .load(get_proc_address, c"glVertexAttribPointer")?;
        self.gl_draw_arrays
            .load(get_proc_address, c"glDrawArrays")?;
        self.gl_disable
            .load(get_proc_address, c"glDisable")?;

        // create the program (vertex + fragment)

        // prepare the buffers
        let mut vertex_array_id = 0;
        self.call_gl_gen_vertex_arrays(1, &mut vertex_array_id);
        dbg!(&vertex_array_id);
        self.call_gl_bind_vertex_array(vertex_array_id);

        let mut vertex_buffer_id = 0;
        self.call_gl_gen_buffers(1, &mut vertex_buffer_id);
        self.check_gl_error(std::line!());

        dbg!(&vertex_buffer_id);
        self.call_gl_bind_buffer(GL_ARRAY_BUFFER, vertex_buffer_id);
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

        Ok(())
        //Err( eyre!("test") )
    }

    fn do_data(&self) {
        let data: &mut [f32] = &mut [
             1.0, -1.0, // top right -> bottom right?
             1.0,  1.0, // top right -> top right?
            -1.0, -1.0, // top left -> bottom left?
            -1.0,  1.0, // top right -> top left?
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
        self.call_gl_buffer_data(
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

        self.call_gl_bind_vertex_array(self.vertex_array_id);
        //dbg!(self.vertex_array_id);
        self.call_gl_bind_buffer(GL_ARRAY_BUFFER, self.vertex_buffer_id);
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
        self.call_gl_enable_vertex_attrib_array(0); // 0 == pos

        self.call_gl_vertex_attrib_pointer(0, 2, GL_FLOAT, GL_FALSE as u8, 0, core::ptr::null());

        //self.do_data();
        //self.call_gl_disable( GL_CULL_FACE );
        self.call_gl_draw_arrays(GL_TRIANGLE_STRIP, 0, 4);
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
        let error = self.call_gl_get_error();
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
