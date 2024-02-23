const XOR128_INV_MAX: f64 = 1.0 / 0xFFFFFFFF_u32 as f64;

pub struct Xor128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Xor128 {
    pub fn new(seed: u32) -> Self {
        Xor128 {
            x: 123456789,
            y: 362436039,
            z: 521288629,
            w: seed,
        }
    }

    pub fn gen(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }

    // [0, a)
    pub fn rand_u(&mut self, a: u32) -> u32 {
        ((self.gen() as u64 * a as u64) >> 32) as u32
    }

    // [a, b)
    pub fn rand_uu(&mut self, a: u32, b: u32) -> u32 {
        self.rand_u(b - a) + a
    }

    // [a, b)
    pub fn rand_ii(&mut self, a: i32, b: i32) -> i32 {
        self.rand_u((b - a) as u32) as i32 + a
    }

    pub fn prob(&mut self) -> f64 {
        self.gen() as f64 * XOR128_INV_MAX
    }

    pub fn rand_ff(&mut self, a: f64, b: f64) -> f64 {
        self.prob() * (b - a) + a
    }
}

#[test]
fn rand_test() {
    let mut mt = Xor128::new(768);
    for _ in 0..100 {
        println!("{}", mt.prob());
    }
    let s = (0..100).map(|_| mt.prob()).sum::<f64>();
    println!("average = {}", s / 100.0);
}
