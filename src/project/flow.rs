use crate::project::Block;
use crate::project::GridPos;
use crate::project::Step;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Flow {
    #[serde(skip)]
    blocks: Vec<Block>,

    #[serde(default)]
    steps: Vec<(Step, GridPos)>,

    #[serde(skip)]
    version: u32,
}

impl Flow {
    pub fn fixup_blocks(&mut self, start: GridPos) -> GridPos {
        let mut p = start;
        for b in self.blocks.iter_mut() {
            p = b.fixup_steps(p);
        }

        // :HACK:
        if !self.blocks.is_empty() {
            self.steps = Default::default();
            for b in self.blocks.iter() {
                for s in b.steps_in_grid().iter() {
                    self.steps.push(s.clone());
                }
            }
        }
        // :TODO: remove blocks
        self.blocks = Default::default();

        // no dirty check needed since we only do this on load anyway
        p
    }

    pub fn steps(&self) -> &Vec<(Step, GridPos)> {
        &self.steps
    }

    pub fn get_step_at(&self, pos: &GridPos) -> Option<&Step> {
        for (s, p) in self.steps.iter() {
            if *p == *pos {
                return Some(s);
            }
        }

        None
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn add_step(&mut self, pos: &GridPos, step: Step) {
        // :TODO: position might already be in use
        if self.steps.iter().any(|(_s, p)| *p == *pos) {
            // position already in used
            eprintln!("Position {pos:?} already in use");
            return;
        }
        self.version += 1;
        self.steps.push((step, pos.clone()));
    }

    pub fn remove_step(&mut self, pos: &GridPos) -> Option<Step> {
        if let Some(idx) = self.steps.iter().position(|(_s, p)| *p == *pos) {
            self.version += 1;
            Some(self.steps.remove(idx).0)
        } else {
            None
        }
    }
    /*
        pub fn blocks(&self) -> &Vec<Block> {
            &self.blocks
        }
    */
    pub fn with_step_at_mut<F>(&mut self, grid_pos: &GridPos, mut f: F)
    where
        F: FnMut(&mut Step) -> (),
    {
        let mut any_changes = false;

        for (s, p) in self.steps.iter_mut() {
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
