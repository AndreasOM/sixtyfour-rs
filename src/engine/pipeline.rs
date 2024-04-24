use crate::engine::gl::*;
use crate::engine::ShaderSource;
use crate::engine::Uniform;
use crate::engine::UniformManager;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::collections::HashMap;
use std::ffi::CString;

#[derive(Debug, Default)]
pub struct Pipeline {
    program: u32,
    uniform_manager: UniformManager,
}

impl Pipeline {
    pub fn setup(&mut self, gl: &mut Gl) -> Result<()> {
        Ok(())
    }
    pub fn bind(&mut self, gl: &mut Gl) -> Result<()> {
        gl.glUseProgram(self.program);
        Ok(())
    }
    pub fn uniform_manager(&self) -> &UniformManager {
        &self.uniform_manager
    }

    pub fn set_property(&mut self, gl: &mut Gl, name: &str, value: f32) -> Result<()> {
        if let Some(u) = self.uniform_manager.get_mut(name) {
            u.set_f32(gl, self.program, value);
        }
        /*
        let n = CString::new(String::from(name))?;
        let l = gl.glGetUniformLocation(self.program, n.as_ptr());
        if l == -1 {
            // eprintln!("Uniform {k} not found");
        } else {
            gl.glProgramUniform1f(self.program, l, value);
        }
        */
        Ok(())
    }
    pub fn rebuild(
        &mut self,
        gl: &mut Gl,
        shader_sources: &mut HashMap<String, ShaderSource>,
    ) -> Result<()> {
        let vertex_shader = {
            let vertex_shader_source = shader_sources
                .get_mut("vertex")
                .expect("Should have vertex shader");

            let vertex_shader = self.compile_shader(gl, vertex_shader_source)?;
            vertex_shader
        };

        let fragment_shader = {
            let fragment_shader_source = shader_sources
                .get_mut("fragment")
                .expect("Should have fragment shader");
            let fragment_shader = self.compile_shader(gl, fragment_shader_source)?;
            fragment_shader
        };
        self.program = self.link_program(gl, vertex_shader, fragment_shader)?;
        eprintln!("pipeline.rebuild() -> success");
        Ok(())
    }
    fn compile_shader(&mut self, gl: &mut Gl, shader_source: &mut ShaderSource) -> Result<GLuint> {
        // :TODO: verify shader type
        let source = CString::new(shader_source.source())?;
        let shader = gl.glCreateShader(shader_source.shader_type());

        gl.glShaderSource(
            shader,
            1,
            &source.as_ptr() as *const *const _,
            core::ptr::null(),
        );
        gl.glCompileShader(shader);

        let mut status: GLint = GL_FALSE as GLint;
        gl.glGetShaderiv(shader, GL_COMPILE_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed compiling shader");
            let mut len = 0;
            gl.glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut len);
            dbg!(len);
            let mut buf = Vec::with_capacity(len as usize);
            unsafe { buf.set_len((len as usize) - 1) };
            let mut len: GLsizei = len;
            gl.glGetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut _);
            let log = String::from_utf8_lossy(&buf);
            dbg!(&log);
            shader_source.set_compile_log_from_string(log.to_string());
            return Err(eyre!("Failed compiling shader!").into());
        } else {
            shader_source.set_compile_log_from_string(String::new());
        }
        gl.check_gl_error(std::line!());

        Ok(shader)
    }

    fn link_program(
        &mut self,
        gl: &mut Gl,
        vertex_shader: u32,
        fragment_shader: u32,
    ) -> Result<u32> {
        let program = gl.glCreateProgram();
        gl.glAttachShader(program, vertex_shader);
        gl.glAttachShader(program, fragment_shader);
        gl.glLinkProgram(program);

        let mut status: GLint = GL_FALSE as GLint;
        gl.glGetProgramiv(program, GL_LINK_STATUS, &mut status);

        dbg!(status);
        if status != GL_TRUE as GLint {
            eprintln!("Failed linking program");
            let mut len = 0;
            gl.glGetProgramiv(program, GL_INFO_LOG_LENGTH, &mut len);
            dbg!(len);
            let mut buf = Vec::with_capacity(len as usize);
            unsafe { buf.set_len((len as usize) - 1) };
            let mut len = len;
            gl.glGetProgramInfoLog(program, len, &mut len, buf.as_mut_ptr() as *mut _);
            let log = String::from_utf8_lossy(&buf);
            dbg!(log);
            return Err(eyre!("Failed linking program").into());
        }
        gl.check_gl_error(std::line!());

        self.program = program;

        let mut params: GLsizei = 0;
        // update uniforms
        gl.glGetProgramiv(self.program, GL_ACTIVE_UNIFORMS, &mut params as *mut _);

        let maxlen = 1024; // :TODO: get longest uniform name
        let mut buf = Vec::with_capacity(maxlen);
        unsafe { buf.set_len((maxlen as usize) - 1) };

        let params = params as GLuint;
        for idx in 0..params {
            //void glGetActiveUniform(GLuint program, GLuint index, GLsizei bufSize, GLsizei *length, GLint *size, GLenum *type,
            //GLchar *name);

            let mut length: GLsizei = 0;
            let mut size: GLint = 0;
            let mut ttype: GLenum = 0;
            unsafe { buf.set_len((maxlen as usize) - 1) };
            gl.glGetActiveUniform(
                self.program,
                idx,
                maxlen as GLsizei,
                &mut length as *mut _,
                &mut size as *mut _,
                &mut ttype as *mut _,
                buf.as_mut_ptr() as *mut _,
            );
            unsafe { buf.set_len(length as usize) };
            let name = String::from_utf8_lossy(&buf);
            let name = name.to_string();

            // Question: Is idx == location?
            let n = CString::new(String::from(&name))?; // what the elf?
            let l = gl.glGetUniformLocation(self.program, n.as_ptr());

            match ttype {
                GL_FLOAT => {
                    let mut u = Uniform::new_float();
                    if l != -1 {
                        u.set_location(l);
                    }
                    self.uniform_manager.add_entry(name.clone(), u);
                }
                GL_FLOAT_VEC3 => {
                    eprintln!("vec3 not implemented");
                }
                o => {
                    eprintln!("Uniform type 0x{o:04x} is not supported");
                }
            }
            dbg!(&name);
        }

        Ok(program)
    }
}
