pub struct FixedQueue<T: Copy, const N: usize> {
    v: [T; N],
    l: usize,
    r: usize,
}

impl<T: Copy, const N: usize> FixedQueue<T, N> {
    pub fn new(init: T) -> Self {
        Self {
            v: [init; N],
            l: 0,
            r: 0,
        }
    }

    pub fn push(&mut self, x: T) {
        self.v[self.r] = x;
        self.r += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.l < self.r {
            self.l += 1;
            Some(self.v[self.l - 1])
        }
        else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.r - self.l
    }
}
