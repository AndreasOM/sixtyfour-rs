#[derive(Debug, Default)]
pub enum Command {
    DeleteProperty {
        name: String,
    },
    LeaveFullscreen,
    #[default]
    Nop,
}
