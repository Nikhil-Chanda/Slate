pub struct Viewport {
    pub row_offset: usize,
    pub col_offset: usize,
    pub height: usize,
    pub width: usize,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            row_offset: 0,
            col_offset: 0,
            height: 0,
            width: 0,
        }
    }
}