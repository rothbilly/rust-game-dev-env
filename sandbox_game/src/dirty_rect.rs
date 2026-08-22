use macroquack::prelude::IVec2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct DirtyRect {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
}

impl DirtyRect {
    const RANGE: i32 = 2;

    pub fn mark_dirty(&mut self, pos: IVec2) {
        let range = DirtyRect::RANGE;
        self.x = self.x.min(pos.x - range);
        self.x2 = self.x2.max(pos.x + range + 1);
        self.y = self.y.min(pos.y - range);
        self.y2 = self.y2.max(pos.y + range + 1);
    }

    pub fn is_empty(&self) -> bool {
        *self == DirtyRect::default()
    }

    pub fn reset(&mut self) {
        *self = DirtyRect::default();
    }
}

impl Default for DirtyRect {
    fn default() -> Self {
        Self {
            x: i32::MAX,
            y: i32::MAX,
            x2: i32::MIN,
            y2: i32::MIN,
        }
    }
}
