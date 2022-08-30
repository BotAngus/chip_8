use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub usize);

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Self(value.into())
    }
}

impl From<usize> for Address {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Address> for usize {
    fn from(address: Address) -> Self {
        address.0
    }
}

impl Add for Address {
    type Output = Address;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Address {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
