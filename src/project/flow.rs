use crate::project::Block;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Flow {
    blocks: Vec<Block>,

    #[serde(skip)]
    version: u32,
}

impl Flow {
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
}
