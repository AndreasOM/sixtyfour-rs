use crate::state::State;

pub trait Window {
    fn name(&self) -> &str;
    fn is_open(&self) -> bool;
    fn update(&mut self, ctx: &egui::Context, state: &mut State);
}
