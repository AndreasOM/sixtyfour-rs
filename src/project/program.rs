use std::path::PathBuf;

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct Program {
    shaders: Vec<Shader>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct Shader {
    shader_type: ShaderType,
    save_path: Option<PathBuf>,

    #[serde(skip)]
    source: String,
    #[serde(skip)]
    unsaved: bool,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub enum ShaderType {
    #[default]
    Fragment,
    Vertex,
}
