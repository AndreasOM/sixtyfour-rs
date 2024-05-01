use crate::project::ResourceId;
use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct Program {
    shaders: Vec<Shader>,
}

impl Program {
    pub fn shaders(&self) -> &Vec<Shader> {
        &self.shaders
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct Shader {
    shader_type: ShaderType,
    resource_id: ResourceId,
}

impl Shader {
    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }
    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Copy, Clone)]
pub enum ShaderType {
    #[default]
    Fragment,
    Vertex,
}
