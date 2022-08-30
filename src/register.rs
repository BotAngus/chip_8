use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

use crate::address::Address;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Registers([u8; 16]);

impl<T> Index<Register<T>> for Registers {
    type Output = u8;

    fn index(&self, index: Register<T>) -> &Self::Output {
        &self.0[index.value.0]
    }
}

impl<T> IndexMut<Register<T>> for Registers {
    fn index_mut(&mut self, index: Register<T>) -> &mut Self::Output {
        &mut self.0[index.value.0]
    }
}

impl Registers {
    pub const fn new() -> Self {
        Self([0; 16])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct X;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Y;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct I;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register<T> {
    value: Address,
    _marker: PhantomData<fn(T) -> T>,
}

impl<T> Register<T> {
    pub fn take(self) -> usize {
        self.value.0
    }
}

impl<T> From<u16> for Register<T> {
    fn from(value: u16) -> Self {
        Self {
            value: value.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<Register<T>> for usize {
    fn from(reg: Register<T>) -> Self {
        reg.value.into()
    }
}

impl<T> From<Address> for Register<T> {
    fn from(value: Address) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}
