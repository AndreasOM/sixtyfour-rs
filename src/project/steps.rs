use crate::project::ResourceId;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub enum Step {
    FullscreenQuad,
    Program {
        resource_id: ResourceId,
        #[serde(skip)]
        version: u32,
    },
    SetUniformF32 {
        name: String,
        value: String,
        version: u32,
    },
    SetUniformF64 {
        name: String,
        value: String,
        version: u32,
    },
    Label {
        name: String,
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
            Self::SetUniformF32 { version, .. } => *version,
            Self::SetUniformF64 { version, .. } => *version,
            Self::Label { version, .. } => *version,
            Self::Nop => 0,
        }
    }
    pub fn types() -> &'static [&'static str] {
        &[
            "FullscreenQuad",
            "Program",
            "SetUniformF32",
            "SetUniformF64",
            "Label",
            "Nop",
        ]
    }
}
impl From<&Step> for String {
    fn from(s: &Step) -> Self {
        match s {
            Step::FullscreenQuad => format!("FullscreenQuad"),
            Step::Program { .. } => format!("Program"),
            Step::SetUniformF32 { .. } => format!("SetUniformF32"),
            Step::SetUniformF64 { .. } => format!("SetUniformF64"),
            Step::Label { .. } => format!("Label"),
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
            "SetUniformF32" => Step::SetUniformF32 {
                name: Default::default(),
                value: Default::default(),
                version: 1,
            },
            "SetUniformF64" => Step::SetUniformF64 {
                name: Default::default(),
                value: Default::default(),
                version: 1,
            },
            "Label" => Step::Label {
                name: Default::default(),
                version: 1,
            },
            "Nop" => Step::Nop,
            _ => Step::Nop,
        }
    }
}
