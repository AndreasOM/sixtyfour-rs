use crate::project::Block;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
pub struct Flow {
    blocks: Vec<Block>,
}

impl Flow {
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }
}
