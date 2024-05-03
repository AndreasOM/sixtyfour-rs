use crate::state::State;

pub trait Window: core::fmt::Debug {
    fn name(&self) -> &str;
    fn is_open(&self) -> bool;
    fn update(&mut self, ctx: &egui::Context, state: &mut State);
    fn serialize(&self) -> String {
        String::default()
    }
    fn deserialize(&mut self, _data: &str) {}
}
