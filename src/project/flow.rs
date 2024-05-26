use crate::project::Block;
use crate::project::GridPos;
use crate::project::Step;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Flow {
    blocks: Vec<Block>,

    #[serde(skip)]
    version: u32,
}

impl Flow {
    pub fn fixup_blocks(&mut self, start: GridPos) -> GridPos {
        let mut p = start;
        for b in self.blocks.iter_mut() {
            p = b.fixup_steps(p);
        }
        // no dirty check needed since we only do this on load anyway
        p
    }

    pub fn get_step_at(&self, pos: &GridPos) -> Option<&Step> {
        for b in self.blocks.iter() {
            if let Some(s) = b.get_step_at(pos) {
                return Some(s);
            }
        }

        None
    }

    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn add_block(&mut self, block: Block) {
        self.version += 1;
        self.blocks.push(block);
    }
    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
    /*
    pub fn blocks_mut(&mut self) -> &mut Vec<Block> {
        &mut self.blocks
    }
    */
    pub fn with_block_mut<F>(&mut self, idx: usize, mut f: F)
    where
        F: FnMut(&mut Block) -> (),
    {
        if let Some(b) = self.blocks.get_mut(idx) {
            let old_version = b.version();
            f(b);
            let new_version = b.version();

            if new_version != old_version {
                self.version += 1;
                eprintln!("Flow version: {}", self.version);
            }
        }
    }
    pub fn with_step_at_mut<F>(&mut self, grid_pos: &GridPos, mut f: F)
    where
        F: FnMut(&mut Step) -> (),
    {
        let mut any_changes = false;

        for b in self.blocks.iter_mut() {
            let old_version = b.version();
            b.with_step_at_mut(grid_pos, &mut f);
            let new_version = b.version();

            // any_changes |= new_version != old_version;

            if new_version != old_version {
                any_changes = true;
            }
        }

        if any_changes {
            self.version += 1;
            eprintln!("Flow version: {}", self.version);
        }
    }
}
