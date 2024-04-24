use crate::engine::gl::*;

#[derive(Default, Debug)]
pub enum UniformType {
    #[default]
    Unknown,
    Float,
}

#[derive(Default, Debug)]
pub struct Uniform {
    location: Option<GLint>,
    ttype: UniformType,
}

impl Uniform {
    pub fn new_float() -> Self {
        Self::new(UniformType::Float)
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
}
