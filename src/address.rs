use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address(pub usize);

impl<U> From<U> for Address
where
    U: Into<usize>,
{
    fn from(value: U) -> Self {
        Self(value.into())
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
