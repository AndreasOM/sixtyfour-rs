use crate::project::Step;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Block {
    name: String,
    steps: Vec<Step>,
}

impl Block {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }
    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }
}
