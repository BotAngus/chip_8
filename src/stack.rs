use crate::address::Address;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stack<const N: usize = 32> {
    inner: [Address; N],
    last: usize,
}

impl<const N: usize> Stack<N> {
    pub fn new() -> Stack<N> {
        Stack {
            inner: [0_usize.into(); N],
            last: 0,
        }
    }
    pub fn push(&mut self, element: Address) {
        self.last += 1;
        self.inner[self.last] = element;
    }

    pub fn pop(&mut self) -> Address {
        let value = self.inner[self.last];
        self.inner[self.last] = 0_usize.into();
        self.last -= 1;
        value.into()
    }
}
