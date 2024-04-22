use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use crate::engine::gl::*;
use color_eyre::Result;
use color_eyre::eyre::eyre;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, Default)]
#[derive(Clone)]
#[serde(default)]
pub struct ShaderSource {
    shader_type: GLenum,
    #[serde(skip)]
    source: String,
    #[serde(skip)]
    dirty: bool,
    #[serde(skip)]
    unsaved: bool,
    save_path: Option<PathBuf>,
}

impl ShaderSource {
    pub fn new(shader_type: GLenum, source: String) -> Self {
        Self {
            shader_type,
            source,
            dirty: false,
            unsaved: false,
            save_path: None,
        }
    }
    pub fn shader_type(&self) -> GLenum {
        self.shader_type
    }
    pub fn dirty(&self) -> bool {
        self.dirty
    }
    pub fn unsaved(&self) -> bool {
        self.unsaved
    }

    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn update_source(&mut self, source: String) {
        self.source = source;
        self.dirty = true;
        self.unsaved = true;
    }
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn save_path(&self) -> Option<&Path> {
        self.save_path.as_deref()
    }

    pub fn set_save_path(&mut self, path: PathBuf ) {
        self.save_path = Some( path );
    }

    pub fn reload(&mut self) -> Result<()> {
        if let Some( path ) = &self.save_path {
            let source = std::fs::read_to_string( path )?;
            self.source = source;
            Ok(())
        } else {
            Err(eyre!("No save path set").into())
        }
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some( path ) = &self.save_path {
            eprintln!("Saving to {path:?}");
            let mut file = std::fs::File::create( path )?;
            file.write_all(self.source.as_bytes())?;
            self.unsaved = false;
            Ok(())
        } else {
            Err(eyre!("No save path set").into())
        }
    }
    pub fn default_file_name(&self) -> String {
        match self.shader_type {
            GL_VERTEX_SHADER => format!("default.vert.glsl" ),
            GL_FRAGMENT_SHADER => format!("default.frag.glsl" ),
            _ => format!("UNDEFINED.glsl"),
        }
    }
}
