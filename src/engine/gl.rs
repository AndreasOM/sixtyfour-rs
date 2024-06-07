#![allow(dead_code)]
use super::gl_wrapper_macro::*;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use core::ffi::*;
use core::mem::transmute;

pub type GLenum = core::ffi::c_uint;
pub type GLuint = core::ffi::c_uint;
pub type GLint = core::ffi::c_int;
pub type GLsizei = core::ffi::c_int;
pub type GLchar = core::ffi::c_char;
pub type GLfloat = core::ffi::c_float;
pub type GLdouble = core::ffi::c_double;

pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
pub const GL_FALSE: GLenum = 0x0000;
pub const GL_TRUE: GLenum = 0x0001;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;

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
    glfp_rects: GlFunctionPointer,
    glfp_gen_buffers: GlFunctionPointer,
    glfp_bind_buffer: GlFunctionPointer,
    glfp_buffer_data: GlFunctionPointer,
    glfp_enable_vertex_attrib_array: GlFunctionPointer,
    glfp_vertex_attrib_pointer: GlFunctionPointer,
    glfp_draw_arrays: GlFunctionPointer,

    glfp_get_shader_info_log: GlFunctionPointer,
    glfp_get_programiv: GlFunctionPointer,
    glfp_get_program_info_log: GlFunctionPointer,
    glfps: Glfps,
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
struct Glfps {
    glGetError: GlFunctionPointer,
    glDisable: GlFunctionPointer,
    glGenVertexArrays: GlFunctionPointer,
    glBindVertexArray: GlFunctionPointer,
    glCreateShader: GlFunctionPointer,
    glShaderSource: GlFunctionPointer,
    glCompileShader: GlFunctionPointer,
    glGetShaderiv: GlFunctionPointer,
    glGetShaderInfoLog: GlFunctionPointer,
    glCreateProgram: GlFunctionPointer,
    glAttachShader: GlFunctionPointer,
    glLinkProgram: GlFunctionPointer,
    glGetProgramiv: GlFunctionPointer,
    glUseProgram: GlFunctionPointer,
    glGetProgramInfoLog: GlFunctionPointer,
    glGetUniformLocation: GlFunctionPointer,
    glProgramUniform1f: GlFunctionPointer,
    glProgramUniform2fv: GlFunctionPointer,
    glProgramUniform3fv: GlFunctionPointer,
    glProgramUniform1d: GlFunctionPointer,

    glGetActiveUniform: GlFunctionPointer,
    glFinish: GlFunctionPointer,
    glGetIntegerv: GlFunctionPointer,
}

impl Gl {
    pub fn check_gl_error(&self, file: &str, line: u32) -> bool {
        let error = self.glGetError();
        match error {
            0 => {
                return false;
            }
            0x500 => {
                eprintln!("GL_INVALID_ENUM - {file}:{line}");
            }
            0x0502 => {
                eprintln!("GL_INVALID_OPERATION - {file}:{line}");
            }
            e => {
                eprintln!("0x{e:04x?} - {file}:{line}");
            }
        }

        true
    }

    pub fn load_all(&mut self, get_proc_address: &dyn Fn(&CStr) -> *const c_void) -> Result<()> {
        //self.glfp_get_error.load(get_proc_address, c"glGetError")?;
        self.glfps
            .glGetError
            .load(get_proc_address, c"glGetError")?;
        self.glfps.glDisable.load(get_proc_address, c"glDisable")?;
        self.glfp_rects.load(get_proc_address, c"glRects")?;
        self.glfps
            .glGenVertexArrays
            .load(get_proc_address, c"glGenVertexArrays")?;
        self.glfps
            .glBindVertexArray
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

        self.glfps
            .glCreateShader
            .load(get_proc_address, c"glCreateShader")?;
        self.glfps
            .glShaderSource
            .load(get_proc_address, c"glShaderSource")?;
        self.glfps
            .glCompileShader
            .load(get_proc_address, c"glCompileShader")?;
        self.glfps
            .glGetShaderiv
            .load(get_proc_address, c"glGetShaderiv")?;
        self.glfps
            .glGetShaderInfoLog
            .load(get_proc_address, c"glGetShaderInfoLog")?;
        self.glfp_get_shader_info_log
            .load(get_proc_address, c"glGetShaderInfoLog")?;
        self.glfps
            .glCreateProgram
            .load(get_proc_address, c"glCreateProgram")?;
        self.glfps
            .glAttachShader
            .load(get_proc_address, c"glAttachShader")?;
        self.glfps
            .glLinkProgram
            .load(get_proc_address, c"glLinkProgram")?;
        self.glfp_get_programiv
            .load(get_proc_address, c"glGetProgramiv")?;
        self.glfp_get_program_info_log
            .load(get_proc_address, c"glGetProgramInfoLog")?;
        self.glfps
            .glGetProgramiv
            .load(get_proc_address, c"glGetProgramiv")?;
        self.glfps
            .glGetProgramInfoLog
            .load(get_proc_address, c"glGetProgramInfoLog")?;
        self.glfps
            .glUseProgram
            .load(get_proc_address, c"glUseProgram")?;
        self.glfps
            .glGetUniformLocation
            .load(get_proc_address, c"glGetUniformLocation")?;
        self.glfps
            .glProgramUniform1f
            .load(get_proc_address, c"glProgramUniform1f")?;
        self.glfps
            .glProgramUniform2fv
            .load(get_proc_address, c"glProgramUniform2fv")?;
        self.glfps
            .glProgramUniform3fv
            .load(get_proc_address, c"glProgramUniform3fv")?;
        self.glfps
            .glProgramUniform1d
            .load(get_proc_address, c"glProgramUniform1d")?;

        self.glfps
            .glGetActiveUniform
            .load(get_proc_address, c"glGetActiveUniform")?;

        self.glfps.glFinish.load(get_proc_address, c"glFinish")?;
        self.glfps
            .glGetIntegerv
            .load(get_proc_address, c"glGetIntegerv")?;

        Ok(())
    }

    create_gl_wrapper!(GLenum glGetError( void ) );
    create_gl_wrapper!(void glDisable( GLenum cap ));
    create_gl_wrapper!(void glGenVertexArrays(GLsizei n, GLuint *arrays));
    create_gl_wrapper!(void glBindVertexArray(GLuint array));

    create_gl_wrapper!(GLuint glCreateShader(GLenum shaderType));
    create_gl_wrapper!(void glShaderSource(GLuint shader, GLsizei count, const GLchar **string, const GLint *length));
    create_gl_wrapper!(void glCompileShader(GLuint shader));
    create_gl_wrapper!(void glGetShaderiv(GLuint shader, GLenum pname, GLint *params));
    create_gl_wrapper!(void glGetShaderInfoLog(GLuint shader, GLsizei maxLength, GLsizei *length, GLchar *infoLog));
    create_gl_wrapper!(GLuint glCreateProgram(void));
    create_gl_wrapper!(void glAttachShader(GLuint program, GLuint shader));
    create_gl_wrapper!(void glLinkProgram(GLuint program));
    create_gl_wrapper!(void glGetProgramiv(GLuint program, GLenum pname, GLint *params));
    create_gl_wrapper!(void glGetProgramInfoLog(GLuint program, GLsizei maxLength, GLsizei *length, GLchar *infoLog));
    create_gl_wrapper!(void glUseProgram(GLuint program));
    create_gl_wrapper!(GLint glGetUniformLocation( GLuint program, const GLchar *name));
    create_gl_wrapper!(void glProgramUniform1f( GLuint program, GLint location, GLfloat v0));
    create_gl_wrapper!(void glProgramUniform2fv( GLuint program, GLint location, GLsizei count, const GLfloat *value));
    create_gl_wrapper!(void glProgramUniform3fv( GLuint program, GLint location, GLsizei count, const GLfloat *value));
    create_gl_wrapper!(void glProgramUniform1d( GLuint program, GLint location, GLdouble v0));
    create_gl_wrapper!(void glGetActiveUniform(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLint *size, GLenum *ttype, GLchar *name));
    create_gl_wrapper!(void glFinish( void ));
    create_gl_wrapper!(void glGetIntegerv( GLenum pname, GLint * data ));

    pub fn rects(&self, x1: i16, y1: i16, x2: i16, y2: i16) {
        unsafe {
            let fn_p = transmute::<
                *const c_void,
                extern "system" fn(c_short, c_short, c_short, c_short) -> c_void,
            >(self.glfp_rects.f);
            fn_p(x1, y1, x2, y2);
        };
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
}
