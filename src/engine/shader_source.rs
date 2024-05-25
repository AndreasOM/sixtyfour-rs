use crate::engine::gl::*;
use crate::project::ResourceId;

//#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
//#[derive(Debug, Default, Clone)]
#[derive(Debug, Default)]
pub struct ShaderSource {
    shader_type: GLenum,
    source: String,
    dirty: bool,

    compile_log: Vec<String>,

    pub last_project_version: u32,
    resource_id: Option<ResourceId>,
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

    pub fn take_compile_log(&mut self) -> Vec<String> {
        //self.compile_log.split_off( 0 )
        let v = core::mem::take(&mut self.compile_log);

        v
    }

    pub fn set_resource_id(&mut self, resource_id: &ResourceId) {
        self.resource_id = Some(resource_id.to_owned())
    }
    pub fn resource_id(&self) -> Option<&ResourceId> {
        self.resource_id.as_ref()
    }
    pub fn take_resource_id(&mut self) -> Option<ResourceId> {
        self.resource_id.take()
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
