use crate::project::ResourceId;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Step {
    FullscreenQuad,
    Program {
        resource_id: ResourceId,
        #[serde(skip)]
        version: u32,
    },
    #[default]
    Nop,
}

impl Step {
    pub fn version(&self) -> u32 {
        match self {
            Self::FullscreenQuad => 0,
            Self::Program { version, .. } => *version,
            Self::Nop => 0,
        }
    }
    pub fn types() -> &'static [&'static str] {
        &["FullscreenQuad", "Program", "Nop"]
    }
}
impl From<&Step> for String {
    fn from(s: &Step) -> Self {
        match s {
            Step::FullscreenQuad => format!("FullscreenQuad"),
            Step::Program { .. } => format!("Program"),
            //Step::Program{ resource_id } => format!("Program {resource_id}"),
            Step::Nop => format!("Nop"),
        }
    }
}

impl From<&str> for Step {
    fn from(s: &str) -> Self {
        match s {
            "FullscreenQuad" => Step::FullscreenQuad,
            "Program" => Step::Program {
                resource_id: Default::default(),
                version: 1,
            },
            "Nop" => Step::Nop,
            _ => Step::Nop,
        }
    }
}
