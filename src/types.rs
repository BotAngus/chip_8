use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(usize);

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value as usize)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct X;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Y;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Register<T> {
    value: usize,
    _marker: PhantomData<fn(T) -> T>,
}

impl From<u16> for Register<X> {
    fn from(value: u16) -> Self {
        Self {
            value: value.into(),
            _marker: PhantomData,
        }
    }
}

impl From<u16> for Register<Y> {
    fn from(value: u16) -> Self {
        Self {
            value: value.into(),
            _marker: PhantomData,
        }
    }
}

impl<T> From<Register<T>> for usize {
    fn from(reg: Register<T>) -> Self {
        reg.value 
    }
}