use std::ops::{Index, IndexMut};

use crate::address::Address;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Memory([u8; 4096]);

impl Memory {
    pub fn new() -> Self {
        Self([0; 4096])
    }

    pub fn load<'a, T: Into<&'a [u8]>>(&mut self, start_at: Address, end_at: Address, slice: T) {
        self.0[start_at.into()..=end_at.into()].copy_from_slice(slice.into())
    }
}

impl Index<Address> for Memory {
    type Output = u8;

    fn index(&self, index: Address) -> &Self::Output {
        let index: usize = index.into();

        &self.0[index]
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        let index: usize = index.into();
        &mut self.0[index]
    }
}
