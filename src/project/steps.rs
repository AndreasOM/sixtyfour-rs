use crate::project::ResourceId;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Step {
    FullscreenQuad,
    Program {
        shaders: Vec<ResourceId>,
    },
    #[default]
    Nop,
}
