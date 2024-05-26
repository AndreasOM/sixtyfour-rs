use crate::project::GridPos;
use crate::project::ResourceId;
use crate::project::ShaderType;

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
    /*
    ChangeFlow {
        flow_command: FlowCommand,
    }
    */
    #[default]
    Nop,
}
