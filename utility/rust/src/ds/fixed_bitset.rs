pub struct BitVec {
    bits: Vec<u64>,
}

impl BitVec {
    pub fn new(n: usize) -> BitVec {
        BitVec {
            bits: vec![0u64; (n + 63) / 64],
        }
    }
    pub fn get(self, i: usize) -> bool {
        self.bits[i >> 6] & (1 << (i & 63)) > 0
    }

    pub fn flip(&mut self, i: usize) {
        self.bits[i >> 6] ^= 1 << (i & 63);
    }

    pub fn set(&mut self, i: usize, b: bool) {
        self.bits[i >> 6] = (self.bits[i >> 6] & !(1 << (i & 63))) | ((b as u64) << (i & 63));
    }
}
