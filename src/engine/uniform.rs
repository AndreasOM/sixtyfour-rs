use crate::engine::gl::*;

#[derive(Default, Debug)]
pub enum UniformType {
    #[default]
    Unknown,
    Float,
    Vec2Float,
    Vec3Float,
}

#[derive(Default, Debug)]
pub struct Uniform {
    location: Option<GLint>,
    ttype: UniformType,
}

impl Uniform {
    pub fn invalidate_location(&mut self) {
        self.location = None;
    }
    pub fn ttype(&self) -> &UniformType {
        &self.ttype
    }
    pub fn new_float() -> Self {
        Self::new(UniformType::Float)
    }
    pub fn new_vec2_float() -> Self {
        Self::new(UniformType::Vec2Float)
    }
    pub fn new_vec3_float() -> Self {
        Self::new(UniformType::Vec3Float)
    }
    pub fn new(ttype: UniformType) -> Self {
        Self {
            location: None,
            ttype,
        }
    }
    pub fn set_location(&mut self, location: GLint) {
        self.location = Some(location);
    }

    pub fn clear_location(&mut self) {
        self.location = None;
    }

    pub fn set_f32(&mut self, gl: &mut Gl, program: u32, value: f32) {
        if let Some(l) = self.location {
            gl.glProgramUniform1f(program, l, value);
        }
    }
    pub fn set_vec2_f32(&mut self, gl: &mut Gl, program: u32, values: &[f32; 2]) {
        if let Some(l) = self.location {
            gl.glProgramUniform2fv(program, l, 1, values.as_ptr());
        }
    }
    pub fn set_vec3_f32(&mut self, gl: &mut Gl, program: u32, values: &[f32; 3]) {
        if let Some(l) = self.location {
            gl.glProgramUniform3fv(program, l, 1, values.as_ptr());
        }
    }
}
