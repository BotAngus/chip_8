#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Display {
    pub data: [[u8; 64]; 128],
}

impl Display {
    pub const fn new() -> Self {
        Self {
            data: [[0; 64]; 128],
        }
    }
}
