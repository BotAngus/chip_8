#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stack<const N: usize = 32> {
    inner: [u16; N],
    last: usize,
}

impl<const N: usize> Stack<N> {
    pub fn new() -> Stack<N> {
        Stack {
            inner: [0; N],
            last: 0,
        }
    }
    pub fn push(&mut self, element: u16) {
        self.last += 1;
        self.inner[self.last] = element;
    }

    pub fn pop(&mut self) -> u16 {
        let value = self.inner[self.last];
        self.inner[self.last] = 0;
        self.last -= 1;
        value
    }
}
