use crate::engine::gl::*;

//#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
//#[derive(Debug, Default, Clone)]
#[derive(Debug, Default)]
pub struct ShaderSource {
    shader_type: GLenum,
    source: String,
    dirty: bool,

    compile_log: Vec<String>,

    pub last_project_version: u32,
}

impl ShaderSource {
    pub fn new(shader_type: GLenum, source: String) -> Self {
        Self {
            shader_type,
            source,
            dirty: false,
            ..Default::default()
        }
    }
    pub fn set_compile_log_from_string(&mut self, compile_log: String) {
        self.compile_log = compile_log.split("\n").map(String::from).collect();
    }
    pub fn compile_log(&self) -> &Vec<String> {
        &self.compile_log
    }

    pub fn shader_type(&self) -> GLenum {
        self.shader_type
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn update_source(&mut self, source: String) {
        self.source = source;
        self.dirty = true;
    }
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}
