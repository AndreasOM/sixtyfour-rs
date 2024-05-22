use crate::project::ResourceId;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Step {
    FullscreenQuad,
    Program {
        resource_id: ResourceId,
    },
    #[default]
    Nop,
}
