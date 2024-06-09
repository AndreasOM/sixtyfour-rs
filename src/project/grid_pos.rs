use core::ops::Add;
use core::ops::Sub;

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
    pub fn inc_x(&mut self) {
        self.x += 1;
    }
    pub fn inc_y(&mut self) {
        self.y += 1;
    }
    pub fn dec_x(&mut self) {
        self.x = self.x.saturating_sub(1);
    }
    pub fn dec_y(&mut self) {
        self.y = self.y.saturating_sub(1);
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

impl Sub for &GridPos {
    type Output = GridPos;

    fn sub(self, other: Self) -> Self::Output {
        GridPos {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}
