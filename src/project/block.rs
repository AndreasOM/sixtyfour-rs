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
    pub fn get_step_at(&self, pos: &GridPos) -> Option<&Step> {
        for (s, p) in self.steps_in_grid.iter() {
            if *p == *pos {
                return Some(s);
            }
        }

        None
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
    pub fn add_step_in_grid(&mut self, pos: &GridPos, step: Step) {
        // :TODO: position might already be in use
        if self.steps_in_grid.iter().any(|(_s, p)| *p == *pos) {
            // position already in used
            eprintln!("Position {pos:?} already in use");
            return;
        }
        self.version += 1;
        self.steps_in_grid.push((step, pos.clone()));
    }
    pub fn remove_step_in_grid(&mut self, pos: &GridPos) -> Option<Step> {
        // :TODO: position might already be in use
        if let Some(idx) = self.steps_in_grid.iter().position(|(_s, p)| *p == *pos) {
            self.version += 1;
            Some(self.steps_in_grid.remove(idx).0)
        } else {
            None
        }
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
    pub fn with_step_at_mut<F>(&mut self, grid_pos: &GridPos, mut f: F)
    where
        F: FnMut(&mut Step) -> (),
    {
        let mut any_changes = false;

        for (s, p) in self.steps_in_grid.iter_mut() {
            if *p == *grid_pos {
                let old_version = s.version();
                f(s);
                let new_version = s.version();

                if new_version != old_version {
                    any_changes = true;
                }
            }
        }

        if any_changes {
            self.version += 1;
            eprintln!("Flow version: {}", self.version);
        }
    }
}
