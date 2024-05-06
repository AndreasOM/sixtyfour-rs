use crate::path_helper::PathHelper;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub type ResourceId = String;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Resource {
    Text(ResourceText),
    Program(ResourceProgram),
    #[default]
    None,
}

impl Resource {
    pub fn reload(&mut self, parent: Option<&Path>) -> bool {
        match self {
            Resource::Text(rt) => rt.reload(parent).is_ok(),
            _ => false,
        }
    }
    pub fn save(&mut self, parent: Option<&Path>) -> Result<()> {
        match self {
            Resource::Text(rt) => rt.save(parent),
            _ => Ok(()),
        }
    }

    pub fn dirty(&self) -> bool {
        match self {
            Resource::Text(rt) => rt.dirty(),
            _ => false,
        }
    }
    pub fn mark_dirty(&mut self) {
        match self {
            Resource::Text(rt) => rt.mark_dirty(),
            _ => {}
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct ResourceText {
    name: String,
    file: Option<PathBuf>,
    #[serde(default)]
    version: u32,

    #[serde(skip)]
    text: String,
    #[serde(skip)]
    dirty: bool,
}

impl ResourceText {
    pub fn file(&self) -> Option<&Path> {
        self.file.as_deref()
    }

    pub fn set_file(&mut self, filename: PathBuf) {
        self.file = Some(filename);
    }

    pub fn name(&mut self) -> &str {
        &self.name
    }
    pub fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn commit_text_change(&mut self) {
        self.version += 1;
    }

    pub fn reload(&mut self, parent: Option<&Path>) -> Result<()> {
        if let Some(path) = &self.file {
            let path = PathHelper::prefix_with(path, parent);
            eprintln!("Loading from {path:?}");
            let text = std::fs::read_to_string(path)?;
            self.text = text;
            self.dirty = false;
            Ok(())
        } else {
            Err(eyre!("No save path set").into())
        }
    }
    pub fn save(&mut self, parent: Option<&Path>) -> Result<()> {
        if let Some(path) = &self.file {
            let path = PathHelper::prefix_with(path, parent);
            eprintln!("Saving to {path:?}");
            let mut file = std::fs::File::create(path)?;
            file.write_all(self.text.as_bytes())?;
            self.dirty = false;
            Ok(())
        } else {
            Err(eyre!("No save path set").into())
        }
    }
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct ResourceProgram {
    shaders: Vec<Shader>,
}

impl ResourceProgram {
    pub fn shaders(&self) -> &Vec<Shader> {
        &self.shaders
    }

    pub fn add_shader(&mut self, shader_type: ShaderType, resource_id: ResourceId) {
        let s = Shader::new(shader_type, resource_id);

        self.shaders.push(s);
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone)]
pub struct Shader {
    shader_type: ShaderType,
    resource_id: ResourceId,
}

impl Shader {
    pub fn new(shader_type: ShaderType, resource_id: ResourceId) -> Self {
        Self {
            shader_type,
            resource_id,
        }
    }
    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }
    pub fn resource_id(&self) -> &ResourceId {
        &self.resource_id
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default, Copy, Clone, PartialEq)]
pub enum ShaderType {
    #[default]
    Fragment,
    Vertex,
}
