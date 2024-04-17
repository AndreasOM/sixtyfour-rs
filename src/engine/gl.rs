use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::*;
use core::mem::transmute;

pub type GLEnum = core::ffi::c_uint;

pub const GL_ARRAY_BUFFER: GLEnum = 0x8892;
pub const GL_STATIC_DRAW: GLEnum = 0x88E4;
pub const GL_DYNAMIC_DRAW: GLEnum = 0x88E8;
pub const GL_FLOAT: GLEnum = 0x1406;
pub const GL_FALSE: GLEnum = 0x0000;
pub const GL_TRIANGLE_STRIP: GLEnum = 0x0005;
pub const GL_CULL_FACE: GLEnum = 0x0B44;

#[derive(Debug)]
pub struct GlFunctionPointer {
    pub f: *const c_void,
}
impl Default for GlFunctionPointer {
    fn default() -> Self {
        Self::null()
    }
}

impl GlFunctionPointer {
    const fn null() -> Self {
        Self {
            f: core::ptr::null(),
        }
    }
}

impl GlFunctionPointer {
    pub fn load(
        &mut self,
        get_proc_address: &dyn Fn(&CStr) -> *const c_void,
        name: &CStr,
    ) -> Result<()> {
        let addr = get_proc_address(name);
        if addr == core::ptr::null() {
            Err(eyre!("Failed loading {name:?}").into())
        } else {
            self.f = addr;
            Ok(())
        }
    }
}

impl From<GlFunctionPointer> for extern "system" fn(i16, i16, i16, i16) -> c_void {
    fn from(p: GlFunctionPointer) -> Self {
        unsafe {
            transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(p.f)
        }
    }
}

//unsafe impl Send for GlFunctionPointer {}
unsafe impl Sync for GlFunctionPointer {}

#[derive(Debug, Default)]
pub struct Gl {
    glfp_get_error: GlFunctionPointer,
    glfp_rects: GlFunctionPointer,
    glfp_gen_vertex_arrays: GlFunctionPointer,
    glfp_bind_vertex_array: GlFunctionPointer,
    glfp_gen_buffers: GlFunctionPointer,
    glfp_bind_buffer: GlFunctionPointer,
    glfp_buffer_data: GlFunctionPointer,
    glfp_enable_vertex_attrib_array: GlFunctionPointer,
    glfp_vertex_attrib_pointer: GlFunctionPointer,
    glfp_draw_arrays: GlFunctionPointer,
    glfp_disable: GlFunctionPointer,

    glfp_create_shader: GlFunctionPointer,
    glfp_shader_source: GlFunctionPointer,
    glfp_compile_shader: GlFunctionPointer,
    glfp_get_shader_iv: GlFunctionPointer,
    glfp_get_shader_info_log: GlFunctionPointer,
    glfp_create_program: GlFunctionPointer,
    glfp_attach_shader: GlFunctionPointer,
    glfp_link_program: GlFunctionPointer,
    glfp_get_program_iv: GlFunctionPointer,
    glfp_get_program_info_log: GlFunctionPointer,
    glfp_use_program: GlFunctionPointer,
}

impl Gl {
    pub fn load_all(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        self.glfp_get_error.load(get_proc_address, c"glGetError")?;
        self.glfp_rects.load(get_proc_address, c"glRects")?;
        self.glfp_gen_vertex_arrays
            .load(get_proc_address, c"glGenVertexArrays")?;
        self.glfp_bind_vertex_array
            .load(get_proc_address, c"glBindVertexArray")?;
        self.glfp_gen_buffers
            .load(get_proc_address, c"glGenBuffers")?;
        self.glfp_bind_buffer
            .load(get_proc_address, c"glBindBuffer")?;
        self.glfp_buffer_data
            .load(get_proc_address, c"glBufferData")?;
        self.glfp_enable_vertex_attrib_array
            .load(get_proc_address, c"glEnableVertexAttribArray")?;
        self.glfp_vertex_attrib_pointer
            .load(get_proc_address, c"glVertexAttribPointer")?;
        self.glfp_draw_arrays
            .load(get_proc_address, c"glDrawArrays")?;
        self.glfp_disable.load(get_proc_address, c"glDisable")?;

        Ok(())
    }

    pub fn get_error(&self) -> core::ffi::c_uint {
        unsafe {
            let fn_p =
                transmute::<*const c_void, extern "system" fn() -> c_uint>(self.glfp_get_error.f);
            fn_p()
        }
    }
    pub fn rects(&self, x1: i16, y1: i16, x2: i16, y2: i16) {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(self.glfp_rects.f);
            fn_p(x1, y1, x2, y2);
        };
    }

    pub fn gen_vertex_arrays(&self, n: c_int, arrays: *mut c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_int, *mut c_uint) -> c_void>(
                self.glfp_gen_vertex_arrays.f,
            );
            fn_p(n, arrays)
        }
    }

    pub fn bind_vertex_array(&self, array: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.glfp_bind_vertex_array.f,
            );
            fn_p(array)
        }
    }

    pub fn gen_buffers(&self, n: c_int, buffers: *mut c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_int, *mut c_uint) -> c_void>(
                self.glfp_gen_buffers.f,
            );
            fn_p(n, buffers)
        }
    }

    pub fn bind_buffer(&self, target: c_uint, buffer: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint, c_uint) -> c_void>(
                self.glfp_bind_buffer.f,
            );
            fn_p(target, buffer)
        }
    }
    // -> GL_ARRAY_BUFFER

    pub fn buffer_data(
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
            >(self.glfp_buffer_data.f);
            fn_p(target, size, data, usage)
        }
    }
    // -> GL_ARRAY_BUFFER == 0x8892, GL_STATIC_DRAW == 0x88E4

    pub fn enable_vertex_attrib_array(&self, index: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.glfp_enable_vertex_attrib_array.f,
            );
            fn_p(index)
        }
    }

    pub fn vertex_attrib_pointer(
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
            >(self.glfp_vertex_attrib_pointer.f);
            fn_p(index, size, ttype, normalized, stride, pointer)
        }
    }

    pub fn draw_arrays(&self, mode: c_uint, first: c_int, count: c_int) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint, c_int, c_int) -> c_void>(
                self.glfp_draw_arrays.f,
            );
            fn_p(mode, first, count)
        }
    }
    // -> GL_TRIANGLE_STRIP == 0x0005

    pub fn disable(&self, mode: c_uint) -> c_void {
        unsafe {
            let fn_p = transmute::<*const c_void, extern "system" fn(c_uint) -> c_void>(
                self.glfp_disable.f,
            );
            fn_p(mode)
        }
    }
}
