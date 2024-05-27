#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct GridPos {
    x: u16,
    y: u16,
}

impl GridPos {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
    pub fn x(&self) -> u16 {
        self.x
    }
    pub fn y(&self) -> u16 {
        self.y
    }
    pub fn x_mut(&mut self) -> &mut u16 {
        &mut self.x
    }
    pub fn y_mut(&mut self) -> &mut u16 {
        &mut self.y
    }
    pub fn inc_y(&mut self) {
        self.y += 1;
    }
}
