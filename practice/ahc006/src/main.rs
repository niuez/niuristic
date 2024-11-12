#![allow(non_snake_case, dead_code)]

const N: usize = 1000;
const N2: usize = 2000;
const M: usize = 50;
const M2: usize = 100;
const S: usize = N2;

fn main() {
    get_time();
    let input = Input::stdin();
    let mut state = State::init(&input);
    state.climb(&input);
    state.output(&input);
}

struct State {
    perm: Vec<usize>, // M2 + 2
    inv: [usize; N2],
    score: i64,
}

impl State {
    fn init(input: &Input) -> State {
        let mut perm = vec![S; M2 + 2];
        let mut inv = [!0; N2];
        let mut score = 0;
        for i in 0..M {
            perm[i + 1] = 2 * i;
            inv[2 * i] = i + 1;
            score += input.dist[perm[i]][perm[i + 1]];
        }
        for i in 0..M {
            perm[M + i + 1] = 2 * i + 1;
            inv[2 * i + 1] = M + i + 1;
            score += input.dist[perm[M + i]][perm[M + i + 1]];
        }
        score += input.dist[perm[M2]][perm[M2 + 1]];
        Self {
            perm, inv, score,
        }
    }

    fn assert_score(&self, input: &Input) {
        let mut score = 0;
        for i in 0..self.perm.len()-1 {
            score += input.dist[self.perm[i]][self.perm[i + 1]];
        }
        assert_eq!(score, self.score);
    }

    fn swap_use_unuse(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let i = mt.gen_range(1..M2 + 1);
        let j = self.inv[self.perm[i] ^ 1];
        let (i, j) = (i.min(j), i.max(j));
        // i is source, j is sink
        let v = self.perm[i] / 2;
        let w = loop {
            let w = mt.gen_range(0..N);
            if self.inv[w * 2] == !0 {
                break w;
            }
        };
        let delta =
            if i + 1 == j {
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[j]]
                - input.dist[self.perm[j]][self.perm[j + 1]]
                + input.dist[self.perm[i - 1]][w * 2]
                + input.dist[w * 2][w * 2 + 1]
                + input.dist[w * 2 + 1][self.perm[j + 1]]
            }
            else {
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                - input.dist[self.perm[j - 1]][self.perm[j]]
                - input.dist[self.perm[j]][self.perm[j + 1]]
                + input.dist[self.perm[i - 1]][w * 2]
                + input.dist[w * 2][self.perm[i + 1]]
                + input.dist[self.perm[j - 1]][w * 2 + 1]
                + input.dist[w * 2 + 1][self.perm[j + 1]]
            };
        if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
            // accept
            self.perm[i] = w * 2;
            self.perm[j] = w * 2 + 1;
            self.inv[v * 2] = !0;
            self.inv[v * 2 + 1] = !0;
            self.inv[w * 2] = i;
            self.inv[w * 2 + 1] = j;
            self.score += delta;
        }
    }

    fn optim_swap_use_unuse(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let i = mt.gen_range(1..M2 + 1);
        let j = self.inv[self.perm[i] ^ 1];
        let (i, j) = (i.min(j), i.max(j));
        // i is source, j is sink
        let v = self.perm[i] / 2;
        let w = loop {
            let w = mt.gen_range(0..N);
            if self.inv[w * 2] == !0 {
                break w;
            }
        };
        let delta =
            if i + 1 == j {
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[j]]
                - input.dist[self.perm[j]][self.perm[j + 1]]
                + input.dist[self.perm[i - 1]][self.perm[j + 1]]
            }
            else {
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                - input.dist[self.perm[j - 1]][self.perm[j]]
                - input.dist[self.perm[j]][self.perm[j + 1]]
                + input.dist[self.perm[i - 1]][self.perm[i + 1]]
                + input.dist[self.perm[j - 1]][self.perm[j + 1]]
            };

        let mut next_perm = self.perm.clone();
        next_perm.remove(j);
        next_perm.remove(i);

        let mut source_min = (std::i64::MAX / 4, !0);
        let mut insert_min = (std::i64::MAX / 4, !0, !0);

        for k in 1..next_perm.len() {
            // ... k-1 [insert!] k ...
            {
                // k-1 [source] [sink] k
                let eta = 0
                    - input.dist[next_perm[k - 1]][next_perm[k]]
                    + input.dist[next_perm[k - 1]][w * 2]
                    + input.dist[w * 2][w * 2 + 1]
                    + input.dist[w * 2 + 1][next_perm[k]];
                insert_min = insert_min.min((eta, k, k));
            }
            {
                // k-1 [sink] k
                let eta = 0
                    - input.dist[next_perm[k - 1]][next_perm[k]]
                    + input.dist[next_perm[k - 1]][w * 2 + 1]
                    + input.dist[w * 2 + 1][next_perm[k]];
                insert_min = insert_min.min((source_min.0 + eta, source_min.1, k));
            }
            {
                // k-1 [source] k
                let eta = 0
                    - input.dist[next_perm[k - 1]][next_perm[k]]
                    + input.dist[next_perm[k - 1]][w * 2]
                    + input.dist[w * 2][next_perm[k]];
                source_min = source_min.min((eta, k));
            }
        }
        let (cost, x, y) = insert_min;
        let delta = delta + cost;
        if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
            next_perm.insert(y, w * 2 + 1);
            next_perm.insert(x, w * 2);
            self.perm = next_perm;
            self.inv[v * 2] = !0;
            self.inv[v * 2 + 1] = !0;
            for k in 1..=M2 {
                self.inv[self.perm[k]] = k;
            }
            self.score += delta;
        }
    }

    fn reverse_range(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let i = mt.gen_range(1..M2 + 1);
        let j = mt.gen_range(1..M2 + 1);
        let (i, j) = (i.min(j), i.max(j) + 1);
        if i + 2 > j {
            return;
        }
        assert!(i + 2 <= j);

        let delta =
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[j - 1]][self.perm[j]]
                + input.dist[self.perm[i - 1]][self.perm[j - 1]]
                + input.dist[self.perm[i]][self.perm[j]]
                ;
        if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
            // check reversible
            let mut temp = vec![false; N];
            let mut ok = true;
            for k in i..j {
                let v = self.perm[k] / 2;
                if temp[v] {
                    ok = false;
                    break;
                }
                temp[v] = true;
            }
            if ok {
                // accept
                self.perm[i..j].reverse();
                for k in i..j {
                    self.inv[self.perm[k]] = k;
                }
                self.score += delta;
            }
        }
        
    }

    fn move_source_index(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let i = mt.gen_range(1..M2 + 1);
        let j = self.inv[self.perm[i] ^ 1];
        let (i, j) = (i.min(j), i.max(j));
        let ni = mt.gen_range(1..j);
        if ni == i || ni >= j { return; }

        // case2:
        // ... i i+1 ... ni ... j
        // ... i+1 ... ni i ... j
        if i < ni {
            let delta =
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                - input.dist[self.perm[ni]][self.perm[ni + 1]]
                + input.dist[self.perm[i - 1]][self.perm[i + 1]]
                + input.dist[self.perm[ni]][self.perm[i]]
                + input.dist[self.perm[i]][self.perm[ni + 1]];
            if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
                self.perm[i..ni+1].reverse();
                self.perm[i..ni].reverse();
                self.score += delta;
                for k in i..ni+1 {
                    self.inv[self.perm[k]] = k;
                }
            }
        }
        // case1:
        //    ... ni ... i-1 i ... j
        // -> ... i ni ... i-1 ... j
        else {
            let delta =
                - input.dist[self.perm[ni - 1]][self.perm[ni]]
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                + input.dist[self.perm[ni - 1]][self.perm[i]]
                + input.dist[self.perm[i]][self.perm[ni]]
                + input.dist[self.perm[i - 1]][self.perm[i + 1]];
            if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
                self.perm[ni..i+1].reverse();
                self.perm[ni+1..i+1].reverse();
                self.score += delta;
                for k in ni..i+1 {
                    self.inv[self.perm[k]] = k;
                }
            }
        }
    }

    fn move_sink_index(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let i = mt.gen_range(1..M2 + 1);
        let j = self.inv[self.perm[i] ^ 1];
        let (i, j) = (i.max(j), i.min(j));
        let ni = mt.gen_range(j+1..M2 + 1);
        if ni == i || ni <= j { return; }

        // case2:
        // j ... i i+1 ... ni ...
        // j ... i+1 ... ni i ...
        if i < ni {
            let delta =
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                - input.dist[self.perm[ni]][self.perm[ni + 1]]
                + input.dist[self.perm[i - 1]][self.perm[i + 1]]
                + input.dist[self.perm[ni]][self.perm[i]]
                + input.dist[self.perm[i]][self.perm[ni + 1]];
            if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
                self.perm[i..ni+1].reverse();
                self.perm[i..ni].reverse();
                self.score += delta;
                for k in i..ni+1 {
                    self.inv[self.perm[k]] = k;
                }
            }
        }
        // case1:
        //    j ... ni ... i-1 i ...
        // -> j ... i ni ... i-1 ...
        else {
            let delta =
                - input.dist[self.perm[ni - 1]][self.perm[ni]]
                - input.dist[self.perm[i - 1]][self.perm[i]]
                - input.dist[self.perm[i]][self.perm[i + 1]]
                + input.dist[self.perm[ni - 1]][self.perm[i]]
                + input.dist[self.perm[i]][self.perm[ni]]
                + input.dist[self.perm[i - 1]][self.perm[i + 1]];
            if delta <= 0 || mt.gen_bool((- delta as f64 / T).exp()) {
                self.perm[ni..i+1].reverse();
                self.perm[ni+1..i+1].reverse();
                self.score += delta;
                for k in ni..i+1 {
                    self.inv[self.perm[k]] = k;
                }
            }
        }
    }
    fn metropolis(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        let query = mt.gen_range(0..3);
        if query == 0 {
            self.reverse_range(input, mt, T);
        }
        else {
            self.optim_swap_use_unuse(input, mt, T);
        }
        /*
        if query == 0 {
            self.swap_use_unuse(input, mt, T)
        }
        else if query == 1 {
            self.reverse_range(input, mt, T);
        }
        else if query == 2 {
            self.move_source_index(input, mt, T);
        }
        else if query == 3 {
            self.move_sink_index(input, mt, T);
        }
        else {
            self.optim_swap_use_unuse(input, mt, T);
        }
        */
    }

    fn climb(&mut self, input: &Input) {
        let T0 = 50.0f64;
        let T1 = 2.0f64;
        let mut T = T0;
        let TL = 1.98;
        let mut iter = 0;
        let mut mt = Mt::new(768);
        loop {
            let t = get_time();
            if t >= TL {
                break;
            }
            iter += 1;
            if iter % 10000 == 0 {
                let p = t / TL;
                T = T0.powf(1.0 - p) * T1.powf(p);
                eprintln!("{} {}", T, self.score);
            }
            self.metropolis(input, &mut mt, T);
        }
    }

    fn output(&self, input: &Input) {
        print!("50");
        //let mut cnt = 0;
        for i in 0..N {
            if self.inv[i * 2] != !0 {
                print!(" {}", i + 1);
                //cnt += 1;
            }
        }
        println!();
        print!("102");
        for v in self.perm.iter().cloned() {
            print!(" {} {}", input.p[v].0, input.p[v].1);
        }
        println!();
    }
}


struct Input {
    p: Vec<(i64, i64)>,
    dist: Vec<Vec<i64>>,
}

impl Input {
    fn stdin() -> Self {
        input! {
            vs: [(i64, i64, i64, i64); N],
        }
        let mut p = vec![];
        for (a, b, c, d) in vs {
            p.push((a, b));
            p.push((c, d));
        }
        p.push((400, 400));
        let mut dist = vec![vec![0i64; N * 2 + 1]; N * 2 + 1];
        for i in 0..N2+1 {
            for j in 0..N2+1 {
                dist[i][j] = (p[i].0 - p[j].0).abs() + (p[i].1 - p[j].1).abs();
            }
        }
        Self { p, dist }
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
