pub struct Cursor {
    pub x: usize,
    pub y: usize,
}

impl Cursor {
    pub fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}