use std::ops::{Index, IndexMut};

use crate::address::Address;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Memory([u8; 4096]);

impl Memory {
    pub fn new() -> Self {
        Self([0; 4096])
    }

    pub fn load<'a, T: Into<&'a [u8]>>(&mut self, start_at: Address, end_at: Address, slice: T) {
        self.0[start_at.0..=end_at.0].copy_from_slice(slice.into())
    }
}

impl<T> Index<T> for Memory
where
    T: Into<usize> + From<Address>,
{
    type Output = u8;

    fn index(&self, index: T) -> &Self::Output {
        let index: usize = index.into();

        &self.0[index]
    }
}

impl<T> IndexMut<T> for Memory
where
    T: Into<usize> + From<Address>,
{
    fn index_mut(&mut self, index: T) -> &mut Self::Output {
        let index: usize = index.into();
        &mut self.0[index]
    }
}
