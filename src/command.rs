use crate::project::GridPos;
use crate::project::GridRect;
use crate::project::ResourceId;
use crate::project::ShaderType;

#[derive(Debug, Default)]
pub enum FlowCommand {
    CloneSteps {
        source_grid_rect: GridRect,
        target_grid_pos: GridPos,
    },
    MoveSteps {
        source_grid_rect: GridRect,
        target_grid_pos: GridPos,
    },
    RemoveSteps {
        grid_rect: GridRect,
    },

    #[default]
    Nop,
}

#[derive(Debug, Default)]
pub enum Command {
    DeleteProperty {
        name: String,
    },
    ToggleFullscreen,
    LeaveFullscreen,
    ProgramAddShader {
        resource_id: ResourceId,
        shader_type: ShaderType,
        shader_resource_id: ResourceId,
    },
    ProgramRemoveShader {
        resource_id: ResourceId,
        shader_resource_id: ResourceId,
    },
    RemoveResource {
        resource_id: ResourceId,
    },
    SelectProgram {
        resource_id: ResourceId,
    },
    // :HACK:
    HackChangeFlowProgramResourceId {
        grid_pos: GridPos,
        resource_id: ResourceId,
    },
    HackAddStepToFlow {
        grid_pos: GridPos,
        step_type: String,
    },
    HackStepSetUniformF32SetNameAndValue {
        grid_pos: GridPos,
        name: String,
        value: String,
    },
    HackStepSetUniformF64SetNameAndValue {
        grid_pos: GridPos,
        name: String,
        value: String,
    },
    HackStepSetUniformVec3F32SetNameAndValues {
        grid_pos: GridPos,
        name: String,
        values: [String;3],
    },
    HackStepLabelSetName {
        grid_pos: GridPos,
        name: String,
    },
    ChangeFlow {
        flow_command: FlowCommand,
    },
    #[default]
    Nop,
}
