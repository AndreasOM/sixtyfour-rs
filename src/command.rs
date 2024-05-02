#[derive(Debug, Default)]
pub enum Command {
    DeleteProperty {
        name: String,
    },
    #[default]
    Nop,
}
