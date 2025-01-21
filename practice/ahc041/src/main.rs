#![allow(non_snake_case, dead_code)]

const N: usize = 1000;
const H: usize = 10;

fn main() {
    get_time();

    /*
    let args: Vec<String> = std::env::args().collect();
    let T0 = args[1].parse::<f64>().unwrap();
    let T1 = args[2].parse::<f64>().unwrap();
    */
    let T0 = 150.20f64;
    let T1 = 1.6f64;

    let input = Input::from_stdin();

    /*
    let mut V = [!0i32; N];
    let mut idx = (0..N).collect_vec();
    idx.sort_by(|i, j| input.A[*i].cmp(&input.A[*j]));

    let mut now_score = 0;
    for i in idx {
        let mut max = 0;
        for &j in input.nexts(i) {
            if V[j] != !0 && V[j] <= 2 {
                max = max.max(V[j] + 1);
            }
        }
        V[i] = max;
        now_score += (V[i] + 1) * input.A[i];
    }
    */
    let mut V = [0; N];
    let mut now_score = input.A.iter().sum::<i32>();
    let mut max_score = now_score;
    let mut max_answer = V.clone();

    let mut neibor_levels = vec![0; N * 12];
    for i in 0..N {
        for &j in input.nexts(i) {
            neibor_levels[i * 12 + V[j] as usize] += 1;
        }
    }

    {
        let mut mt = Mt::new(768);
        // let T0 = 200.0f64;
        // let T1 = 20.0f64;
        let mut T = T0;
        let mut iter = 0;
        let TL = 1.998;

        'LOOP: loop {
            let t = get_time();
            if t >= TL {
                break;
            }
            iter += 1;
            if (iter & ((1 << 20) - 1)) == 0 {
                eprintln!("{} {} {}", iter, T, now_score);
                let p = t / TL;
                // T = T0.powf(1.0 - p) * T1.powf(p);
                T = T0 * (1.0 - p) + T1 * p;
            }
            let query = mt.gen_range(0.0..1.0);
            if query <= 0.50 {
                let i = mt.gen_range(0..N);
                let L = input.S[i + 1] - input.S[i];
                let chi = mt.gen_range(0..=L);
                if chi < L && V[input.G[input.S[i] + chi]] == 10 {
                    continue 'LOOP;
                }
                let before_v = V[i];
                V[i] = if chi < L { V[input.G[input.S[i] + chi]] + 1 } else { 0 };

                /*
                if neibor_levels[i * 12 + V[i] as usize - 1] == 0 {
                    V[i] = before_v;
                    continue 'LOOP;
                }
                */

                let delta = input.A[i] * (V[i] - before_v) as i32;
                if delta >= 0 || mt.gen_bool((delta as f64 / T).exp()) {
                }
                else {
                    // not accept
                    V[i] = before_v;
                    continue 'LOOP;
                }

                // are the neiborhoods valid ?
                {
                    for &j in input.nexts(i) {
                        if V[j] == 0 || V[j] - before_v != 1 { continue; } // PLAY OF THE GAME
                        // V[j] == before_v + 1
                        // need before_v
                        if neibor_levels[j * 12 + before_v as usize] <= 1 {
                            V[i] = before_v;
                            continue 'LOOP
                        }
                    }
                }

                {
                    now_score += delta;
                    for &j in input.nexts(i) {
                        neibor_levels[j * 12 + before_v as usize] -= 1;
                        neibor_levels[j * 12 + V[i] as usize] += 1;
                    }
                    // eprintln!("{} {}", i, V[i]);
                }
            }
            else {
                let (i, j) = input.E[mt.gen_range(0..input.M)];
                if V[i] == V[j] { continue; }

                /*
                if neibor_levels[i * 12 + V[i] as usize - 1] == 0 {
                    V[i] = before_v;
                    continue 'LOOP;
                }
                */
                if V[i] - V[j] != 1 && V[i] > 0 && neibor_levels[j * 12 + V[i] as usize - 1] == 0 {
                    continue 'LOOP;
                }
                if V[j] - V[i] != 1 && V[j] > 0 && neibor_levels[i * 12 + V[j] as usize - 1] == 0 {
                    continue 'LOOP;
                }

                let delta = input.A[i] * (V[j] - V[i]) + input.A[j] * (V[i] - V[j]);
                if delta >= 0 || mt.gen_bool((delta as f64 / T).exp()) {
                }
                else {
                    // not accept
                    continue 'LOOP;
                }

                // are the neiborhoods valid ?
                {
                    for &k in input.nexts(i) {
                        if k == j || V[k] == 0 { continue; }
                        if V[k] - V[i] == 1 {
                            if neibor_levels[k * 12 + V[i] as usize] <= 1 && !input.B.get(k * N + j) {
                                continue 'LOOP;
                            }
                        }
                    }
                }
                {
                    for &k in input.nexts(j) {
                        if k == i || V[k] == 0 { continue; }
                        if V[k] - V[j] == 1 {
                            if neibor_levels[k * 12 + V[j] as usize] <= 1 && !input.B.get(k * N + i) {
                                continue 'LOOP;
                            }
                        }
                    }
                }

                {
                    for &k in input.nexts(i) {
                        neibor_levels[k * 12 + V[i] as usize] -= 1;
                        neibor_levels[k * 12 + V[j] as usize] += 1;
                    }
                    for &k in input.nexts(j) {
                        neibor_levels[k * 12 + V[j] as usize] -= 1;
                        neibor_levels[k * 12 + V[i] as usize] += 1;
                    }
                    V.swap(i, j);
                    now_score += delta;
                    // eprintln!("{} {}", i, V[i]);
                    /*
                    eprintln!("i {} {}", i, V[i]);
                    for &k in input.nexts(i) {
                        eprintln!("{} {}", k, V[k]);
                    }
                    eprintln!("j {} {}", j, V[j]);
                    for &k in input.nexts(j) {
                        eprintln!("{} {}", k, V[k]);
                    }

                    {
                        for i in 0..N {
                            if V[i] == 0 {
                                continue;
                            }
                            let mut ok = false;
                            for &j in input.nexts(i) {
                                if V[i] - V[j] == 1  {
                                    ok = true;
                                }
                            }
                            if !ok {
                                eprintln!("no {} {}", i, V[i]);
                                for &j in input.nexts(i) {
                                    eprintln!("{} {}", j, V[j]);
                                }
                                assert!(false);
                            }
                        }
                    }
                    */
                }
            }
        }
    }
    max_score = now_score;
    max_answer = V.clone();

    eprintln!("{}", max_score);
    V = max_answer;

    let mut P = [!0; N];

    for &(u, v) in &input.E {
        if V[u] + 1 == V[v] {
            P[v] = u;
        }
        if V[v] + 1 == V[u] {
            P[u] = v;
        }
    }

    for i in 0..N {
        print!("{} ", if P[i] == !0 { -1 } else { P[i] as isize });
    }
    println!();
    /*
    for i in 0..N {
        eprintln!("{} {}", i, V[i]);
    }
    */
}

struct Input {
    M: usize,
    A: [i32; N],
    E: Vec<(usize, usize)>,
    P: Vec<(usize, usize)>,
    G: Vec<usize>,
    S: [usize; N + 1],
    B: Bitset,
}

impl Input {
    fn from_stdin() -> Input {
        input! {
            _n: usize,
            M: usize,
            _h: usize,
            A: [i32; N],
            E: [(usize, usize); M],
            P: [(usize, usize); N],
        }
        let mut g = vec![vec![]; N];
        let mut B = Bitset::new(N * N);
        for i in 0..M {
            let (u, v) = E[i];
            B.set(u * N + v, true);
            B.set(v * N + u, true);
            g[u].push(v);
            g[v].push(u);
        }
        let mut G = vec![];
        G.reserve(M * 2);
        let mut S = [0; N + 1];
        for i in 0..N {
            for &j in &g[i] {
                G.push(j);
            }
            S[i + 1] = G.len();
        }
        Input {
            M,
            A: A.try_into().unwrap(),
            E,
            P,
            G,
            S,
            B,
        }
    }

    #[inline(always)]
    fn nexts(&self, i: usize) -> &[usize] {
        &self.G[self.S[i]..self.S[i + 1]]
    }
}

use itertools::Itertools;
use proconio::{input, input_interactive};
use rand::prelude::*;
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

use std::cell::RefCell;

pub struct UnionFind {
    par: RefCell<Vec<usize>>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: RefCell::new((0..n).collect()),
            sz: std::iter::repeat(1).take(n).collect(),
        }
    }

    pub fn root(&self, v: usize) -> usize {
        let p = self.par.borrow()[v];
        if p == v { v }
        else {
            let r = self.root(p);
            self.par.borrow_mut()[v] = r;
            r
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) -> Option<(usize, usize)> {
        let a = self.root(a);
        let b = self.root(b);
        if a == b { None }
        else if self.sz[a] < self.sz[b] {
            self.par.borrow_mut()[a] = b;
            self.sz[b] += self.sz[a];
            Some((b, a))
        }
        else {
            self.par.borrow_mut()[b] = a;
            self.sz[a] += self.sz[b];
            Some((a, b))
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
