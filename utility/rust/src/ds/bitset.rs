#[derive(Clone)]
pub struct BitVec {
    n: usize,
    bits: Vec<u64>,
}

impl std::fmt::Debug for BitVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = (0..self.n)
            .map(|i| if self.get(i) { '1' } else { '0' })
            .collect();
        write!(f, "[{}]", s)
    }
}

impl BitVec {
    pub fn new(n: usize) -> BitVec {
        BitVec {
            n,
            bits: vec![0u64; (n + 63) / 64],
        }
    }
    pub fn size(&self) -> usize {
        self.n
    }
    pub fn get(&self, i: usize) -> bool {
        self.bits[i >> 6] & (1 << (i & 63)) > 0
    }

    pub fn flip(&mut self, i: usize) {
        self.bits[i >> 6] ^= 1 << (i & 63);
    }

    pub fn set(&mut self, i: usize, b: bool) {
        self.bits[i >> 6] = (self.bits[i >> 6] & !(1 << (i & 63))) | ((b as u64) << (i & 63));
    }
}

#[test]
fn bitset_test() {
    let mut b = BitVec::new(70);
    assert_eq!(b.bits.len(), 2);
    b.set(5, true);
    assert_eq!(b.get(4), false);
    assert_eq!(b.get(5), true);
    b.flip(69);
    assert_eq!(b.get(69), true);
    assert_eq!(format!("{:?}", b), "[0000010000000000000000000000000000000000000000000000000000000000000001]");
}
