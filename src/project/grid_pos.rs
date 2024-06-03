use core::ops::Add;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize, Copy, Clone, PartialEq)]
pub struct GridPos {
    x: u16,
    y: u16,
}

impl GridPos {
    pub const fn zero() -> &'static Self {
        &Self { x: 0, y: 0 }
    }
    pub const fn one() -> &'static Self {
        &Self { x: 1, y: 1 }
    }

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

impl Add for &GridPos {
    type Output = GridPos;

    fn add(self, other: Self) -> Self::Output {
        GridPos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
