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

    pub fn from_top_left_with_size_one(top_left: &GridPos) -> Self {
        Self {
            top_left: top_left.clone(),
            bottom_right: top_left + GridPos::one(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top_left.x() >= self.bottom_right.x() || self.top_left.y() >= self.bottom_right.y()
    }
    pub fn set_top(&mut self, top: u16) {
        *self.top_left.y_mut() = top;
    }
    pub fn set_bottom_inclusive(&mut self, bottom: u16) {
        *self.bottom_right.y_mut() = bottom + 1;
    }
    pub fn set_left(&mut self, left: u16) {
        *self.top_left.x_mut() = left;
    }
    pub fn set_right_inclusive(&mut self, right: u16) {
        *self.bottom_right.x_mut() = right + 1;
    }
    pub fn set_top_left(&mut self, top_left: &GridPos) {
        self.top_left = *top_left;
    }
    pub fn set_bottom_right(&mut self, bottom_right: &GridPos) {
        self.bottom_right = *bottom_right;
    }
    pub fn set_bottom_right_inclusive(&mut self, bottom_right: &GridPos) {
        self.bottom_right = bottom_right + GridPos::one();
    }
    pub fn set_size(&mut self, size: &GridPos) {
        self.bottom_right = &self.top_left + &size;
    }
    pub fn size(&self) -> GridPos {
        &self.bottom_right - &self.top_left
    }
    pub fn top_left(&self) -> &GridPos {
        &self.top_left
    }
    pub fn top_left_mut(&mut self) -> &mut GridPos {
        &mut self.top_left
    }
    pub fn bottom_right_exclusive(&self) -> &GridPos {
        &self.bottom_right
    }

    // Note: Only use this if we are sure you know what you are doing ;) // :FIXME: docs
    pub fn bottom_right(&self) -> Option<GridPos> {
        if self.is_empty() {
            None
        } else {
            Some(&self.bottom_right - GridPos::one())
        }
    }

    pub fn union_with_pos(&self, pos: &GridPos) -> GridRect {
        let min_x = self.top_left.x().min(pos.x());
        let min_y = self.top_left.y().min(pos.y());
        let max_x = self.bottom_right.x().max(pos.x() + 1);
        let max_y = self.bottom_right.y().max(pos.y() + 1);

        Self {
            top_left: GridPos::new(min_x, min_y),
            bottom_right: GridPos::new(max_x, max_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_works() {
        let gp_00 = GridPos::new(0, 0);
        let gp_11 = GridPos::new(1, 1);
        let gp_22 = GridPos::new(2, 2);
        let gr = GridRect::default();

        let gr = gr.union_with_pos(&gp_00);
        assert_eq!((gp_00, gp_11), (gr.top_left, gr.bottom_right));
        assert!(!gr.is_empty());
    }

    #[test]
    fn sizing_works() {
        let gp_00 = GridPos::new(0, 0);
        let gp_11 = GridPos::new(1, 1);
        let gp_22 = GridPos::new(2, 2);
        let mut gr = GridRect::default();

        assert!(gr.is_empty());
        assert_eq!((gp_00, gp_00), (gr.top_left, gr.bottom_right));

        gr.set_size(&gp_00);
        assert!(gr.is_empty());
        assert_eq!((gp_00, gp_00), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_00, gr.size());

        gr.set_size(&gp_11);
        assert_eq!((gp_00, gp_11), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_11, gr.size());

        gr.set_size(&gp_22);
        assert_eq!(gp_22, gr.size());
        assert_eq!((gp_00, gp_22), (gr.top_left, gr.bottom_right));

        gr.set_bottom_right(&gp_00);
        assert!(gr.is_empty());
        assert_eq!((gp_00, gp_00), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_00, gr.size());

        gr.set_bottom_right(&gp_11);
        assert!(!gr.is_empty());
        assert_eq!((gp_00, gp_11), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_11, gr.size());

        gr.set_bottom_right(&gp_22);
        assert!(!gr.is_empty());
        assert_eq!((gp_00, gp_22), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_22, gr.size());

        gr.set_bottom_right_inclusive(&gp_11);
        assert!(!gr.is_empty());
        assert_eq!((gp_00, gp_22), (gr.top_left, gr.bottom_right));
        assert_eq!(gp_22, *gr.bottom_right_exclusive());
        assert_eq!(gp_22, gr.size());
    }
}
