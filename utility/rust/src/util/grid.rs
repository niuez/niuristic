#![allow(non_snake_case, dead_code)]


#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct N2pPos<const N: usize, const P: usize, const L: usize>(pub usize);

impl<const N: usize, const P: usize, const L: usize> N2pPos<N, P, L> {
    // lurd
    const D: [usize; 4] = [1usize.wrapping_neg(), P.wrapping_neg(), 1, P];

    pub fn dang() -> Self {
        Self(!0)
    }

    pub fn is_dang(&self) -> bool {
        self.0 == !0
    }

    pub fn xy(i: usize, j: usize) -> Self {
        Self(i * P + j)
    }

    pub fn to_xy(&self) -> (usize, usize) {
        (self.0 >> L, self.0 & (P - 1))
    }

    pub fn delta(&self, di: usize) -> Option<Self> {
        let v = self.0.wrapping_add(Self::D[di]);
        if v & (P - 1) < N && v < N * P {
            Some(Self(v))
        }
        else {
            None
        }
    }
}

impl<const N: usize, const P: usize, const L: usize> Default for N2pPos<N, P, L> {
    fn default() -> Self {
        Self::dang()
    }
}




// N2pが簡易的かつ最速

fn Np1(N: usize, T: usize) {
    let start_time = get_time();
    let Np = N + 1;
    let mut mt = Mt::new(768);
    let mut X = vec![0usize; Np * N];
    let D = vec![Np.wrapping_neg(), Np, 1usize.wrapping_neg(), 1];
    let V = (0..N*N).map(|i| (i / N) * Np + i % N).collect_vec();

    /*
    for &i in &V {
        eprint!("{} {} -> ", i / Np, i % Np);
        for &d in &D {
            let j = i.wrapping_add(d);
            if j % Np != Np - 1 && j < X.len() {
                eprint!("{} {} ", j / Np, j % Np);
            }
        }
        eprintln!();
    }
    */

    for _ in 0..T {
        let v = mt.gen_range(0..N*N);
        let i = V[v];
        for &d in &D {
            let j = i.wrapping_add(d);
            if j % Np != Np - 1 && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }
    }

    let end_time = get_time();
    eprintln!("Np1 {} {}", end_time - start_time, X[0]);
}

fn N2p(N: usize, T: usize) {
    let start_time = get_time();
    let Np = (N + 1).next_power_of_two();
    let mut mt = Mt::new(768);
    let mut X = vec![0usize; Np * N];
    let D = [Np.wrapping_neg(), Np, 1usize.wrapping_neg(), 1];
    let V = (0..N*N).map(|i| (i / N) * Np + i % N).collect_vec();

    for _ in 0..T {
        let v = mt.gen_range(0..N*N);
        let i = V[v];
        for &d in &D {
            let j = i.wrapping_add(d);
            if j & (Np - 1) < N && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }
    }

    let end_time = get_time();
    eprintln!("N2p {} {}", end_time - start_time, X[0]);
}

fn N2p_unroll(N: usize, T: usize) {
    let start_time = get_time();
    let Np = (N + 1).next_power_of_two();
    let mut mt = Mt::new(768);
    let mut X = vec![0usize; Np * N];
    let V = (0..N*N).map(|i| (i / N) * Np + i % N).collect_vec();

    for _ in 0..T {
        let v = mt.gen_range(0..N*N);
        let i = V[v];
        {
            let j = i.wrapping_sub(Np);
            if j & (Np - 1) < N && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }
        {
            let j = i.wrapping_add(Np);
            if j & (Np - 1) < N && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }
        {
            let j = i.wrapping_sub(1);
            if j & (Np - 1) < N && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }
        {
            let j = i.wrapping_add(1);
            if j & (Np - 1) < N && j < X.len() {
                X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
            }
        }

        /* 速度は同じくらい
        if i >= Np {
            let j = i.wrapping_sub(Np);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if i + Np < X.len() {
            let j = i.wrapping_add(Np);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if i & (Np - 1) > 0 {
            let j = i.wrapping_sub(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if (i & (Np - 1)) + 1 < N {
            let j = i.wrapping_add(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        */
    }

    let end_time = get_time();
    eprintln!("N2p unroll {} {}", end_time - start_time, X[0]);
}

fn bitvector_unroll(N: usize, T: usize) {
    let start_time = get_time();
    let mut mt = Mt::new(768);
    let mut X = vec![0usize; N * N];
    let mut bit = Bitset::new(N * N * 4);

    for i in 0..N {
        for j in 0..N {
            let v = i * N + j;
            if i > 0 {
                bit.set(v * 4 + 0, true);
            }
            if i + 1 < N {
                bit.set(v * 4 + 1, true);
            }
            if j > 0 {
                bit.set(v * 4 + 2, true);
            }
            if j + 1 < N {
                bit.set(v * 4 + 3, true);
            }
        }
    }

    for _ in 0..T {
        let i = mt.gen_range(0..N*N);
        if bit.get(i * 4 + 0) {
            let j = i.wrapping_sub(N);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }
        if bit.get(i * 4 + 1) {
            let j = i.wrapping_add(N);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }
        if bit.get(i * 4 + 2) {
            let j = i.wrapping_sub(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }
        if bit.get(i * 4 + 3) {
            let j = i.wrapping_add(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }

        /* 速度は同じくらい
        if i >= Np {
            let j = i.wrapping_sub(Np);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if i + Np < X.len() {
            let j = i.wrapping_add(Np);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if i & (Np - 1) > 0 {
            let j = i.wrapping_sub(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        if (i & (Np - 1)) + 1 < N {
            let j = i.wrapping_add(1);
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * v);
        }
        */
    }

    let end_time = get_time();
    eprintln!("bitvector {} {}", end_time - start_time, X[0]);
}

fn fillvector_delta(N: usize, T: usize) {
    let start_time = get_time();

    let mut mt = Mt::new(768);
    let mut X = vec![0usize; N * N];

    let mut delta = vec![!0; N * N * 4];
    for i in 0..N {
        for j in 0..N {
            let v = i * N + j;
            if i > 0 {
                delta[v * 4 + 0] = v - N;
            }
            if i + 1 < N {
                delta[v * 4 + 1] = v + N;
            }
            if j > 0 {
                delta[v * 4 + 2] = v - 1;
            }
            if j + 1 < N {
                delta[v * 4 + 3] = v + 1;
            }
        }
    }
    /*
    for i in 0..N*N {
        eprint!("{} {} -> ", i / N, i % N);
        for &j in delta[i * 4..i * 4 + 4].iter().filter(|x| **x != !0) {
            eprint!("{} {} ", j / N, j % N);
        }
        eprintln!();
    }
    */
    for _ in 0..T {
        let i = mt.gen_range(0..N*N);
        for &j in delta[i * 4..i * 4 + 4].iter().filter(|x| **x != !0) {
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }
    }

    let end_time = get_time();
    eprintln!("vector fill delta {} {}", end_time - start_time, X[0]);
}

fn vector_delta(N: usize, T: usize) {
    let start_time = get_time();

    let mut mt = Mt::new(768);
    let mut X = vec![0usize; N * N];

    let mut delta = vec![vec![]; N * N];
    for i in 0..N {
        for j in 0..N {
            let v = i * N + j;
            if i > 0 {
                delta[v].push(v - N);
            }
            if i + 1 < N {
                delta[v].push(v + N);
            }
            if j > 0 {
                delta[v].push(v - 1);
            }
            if j + 1 < N {
                delta[v].push(v + 1);
            }
        }
    }
    for _ in 0..T {
        let i = mt.gen_range(0..N*N);
        for &j in &delta[i] {
            X[j] = X[j].wrapping_add(mt.gen_range(0..128) * i);
        }
    }

    let end_time = get_time();
    eprintln!("vector delta {} {}", end_time - start_time, X[0]);
}

use itertools::Itertools;
use proconio::{input, input_interactive};
use rand::prelude::*;
use rand_distr::uniform::{UniformInt, UniformSampler};
use rand_pcg::Pcg64Mcg;
type Mt = Pcg64Mcg;

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

pub struct Bitset {
    b: Vec<u64>,
    n: usize,
}

impl Bitset {
    pub fn new(n: usize) -> Self {
        let s = (n + 63) / 64;
        Self { b: std::iter::repeat(0).take(s).collect(), n }
    }

    pub fn set(&mut self, i: usize, v: bool) {
        assert!(i < self.n);
        match v {
            true => self.b[i / 64] |= 1u64 << (i & 63),
            false => self.b[i / 64] &= !(1u64 << (i & 63)),
        }
    }

    pub fn get(&self, i: usize) -> bool {
        assert!(i < self.n);
        (self.b[i / 64] & (1u64 << (i & 63))) > 0
    }
}
