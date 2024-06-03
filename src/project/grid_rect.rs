use crate::project::GridPos;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct GridRect {
    top_left: GridPos,
    bottom_right: GridPos,
}

impl GridRect {
    pub fn new(top_left: GridPos, bottom_right: GridPos) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
    pub fn set_top_left(&mut self, top_left: &GridPos) {
        self.top_left = *top_left;
    }
    pub fn set_size(&mut self, size: &GridPos) {
        self.bottom_right = &self.top_left + size;
    }
    pub fn top_left(&self) -> &GridPos {
        &self.top_left
    }
    pub fn top_left_mut(&mut self) -> &mut GridPos {
        &mut self.top_left
    }
    pub fn bottom_right(&self) -> &GridPos {
        &self.bottom_right
    }
}
