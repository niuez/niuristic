#![allow(non_snake_case, dead_code)]

const N: usize = 30;
const FIXED: usize = 8;

fn main() {
    get_time();
}
// RLDU
const DIR: [usize; 4] = [1, 1usize.wrapping_neg(), N, N.wrapping_neg()];
const TILE_OUT_DIR: [[usize; 4]; 8] = [
    [3, !0, 1, !0],
    [2, !0, !0, 1],
    [!0, 2, !0, 0],
    [!0, 3, 0, !0],
    [3, 2, 1, 0],
    [2, 3, 0, 1],
    [0, 1, !0, !0],
    [!0, !0, 2, 3],
];
const TILE_FIRST_DIR: [usize; 8] = [3, 2, 2, 3, 3, 2, 0, 2];
const DELTA: Lazy<Vec<Vec<usize>>> = Lazy::new(|| {
    let mut ans = vec![vec![!0; 4]; N*N];
    for i in 0..N {
        for j in 0..N {
            let v = i * N + j;
            if j + 1 < N { ans[v][0] = v.wrapping_add(DIR[0]); }
            if j == 0 { ans[v][1] = v.wrapping_add(DIR[1]); }
            if i + 1 < N { ans[v][2] = v.wrapping_add(DIR[2]); }
            if i == 0 { ans[v][3] = v.wrapping_add(DIR[3]); }
        }
    }
    ans
});

struct DFS {
    road: Vec<(usize, usize, usize)>,
    end: (usize, usize),
    thre: f64,
    tl: f64,
}

impl DFS {
    // -> terminated?
    fn dfs(&mut self) -> bool {
    }
}

struct State {
    tile: Vec<usize>,
    rings: [Vec<(usize, usize)>; 2],
}

impl State {
    fn stdin() -> Self {
        use proconio::marker::Chars;
        input! {
            t: [Chars; N],
        }
        let tile = t.into_iter().map(
            |s| s.into_iter().map(|c| c.to_digit(10).unwrap() as usize).collect_vec()
        ).concat();
        State {
            tile,
            rings: [Vec::new(), Vec::new()],
        }
    }

    fn init_ring(&mut self) {
        let mut rings = vec![];
        let mut used = vec![false; N*N];
        for i in 0..N*N {
            if used[i] { continue; }
            let mut v = i;
            let mut d = TILE_FIRST_DIR[self.tile[i]] ^ 1;
            let mut ring = vec![(v, d)];
            let mut ok = false;
            used[v] = true;
            loop {
                let nd = TILE_OUT_DIR[self.tile[v]][d];
                assert!(nd == !0);
                let u = DELTA[v][nd];
                if i == u {
                    ok = true;
                    break;
                }
                if u == !0 || TILE_OUT_DIR[self.tile[u]][nd] == !0 {
                    break;
                }
                v = u;
                d = nd;
                ring.push((v, d));
                used[v] = true;
            }
            if ok {
                let score = ring.iter().map(|&(v, _)| (v / N).abs_diff(N / 2) + (v % N).abs_diff(N / 2)).min().unwrap();
                rings.push((score, ring));
            }
        }

        rings.sort_by(|x, y| x.0.cmp(&y.0));
        self.rings = [rings[0].1.clone(), rings[1].1.clone()];
        for t in 0..2 {
            for x in self.rings[t].iter() {
                self.tile[x.0] |= FIXED;
            }
        }
    }
}



use itertools::Itertools;
use once_cell::sync::Lazy;
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
