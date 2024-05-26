use crate::project::GridPos;
use crate::project::Step;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Block {
    name: String,
    steps: Vec<Step>,
    #[serde(default)]
    steps_in_grid: Vec<(Step, GridPos)>,

    #[serde(skip)]
    version: u32,
}

impl Block {
    pub fn fixup_steps(&mut self, start: GridPos) -> GridPos {
        if self.steps.len() > self.steps_in_grid.len() {
            let mut p = start;
            for s in self.steps.drain(..) {
                self.steps_in_grid.push((s, p.clone()));
                p.inc_y();
            }

            p
        } else {
            start
        }
    }
    pub fn version(&self) -> u32 {
        self.version
    }
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
    /*
    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }
    */
    pub fn steps_in_grid(&self) -> &Vec<(Step, GridPos)> {
        &self.steps_in_grid
    }
    /*
    pub fn steps_mut(&mut self) -> &mut Vec<Step> {
        &mut self.steps
    }
    */
    pub fn with_step_mut<F>(&mut self, idx: usize, mut f: F)
    where
        F: FnMut(&mut Step) -> (),
    {
        if let Some((s, _gp)) = self.steps_in_grid.get_mut(idx) {
            let old_version = s.version();
            f(s);
            let new_version = s.version();

            if new_version != old_version {
                self.version += 1;
                eprintln!("Block version: {}", self.version);
            }
        }
    }
}
