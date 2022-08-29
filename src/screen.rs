#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Screen {
    pub data: [[u8; 64]; 128],
}

impl Screen {
    pub const fn new() -> Self {
        Self {
            data: [[0; 64]; 128],
        }
    }
}
