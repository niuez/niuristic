#![allow(non_snake_case, dead_code)]

fn main() {
    get_time();
    let input = Input::stdin(true);

    let rects = input.local.actual.clone();
}

#[derive(Clone)]
struct Rect {
    w: i32,
    h: i32,
}

#[derive(Clone)]
struct Local {
    actual: Vec<Rect>,
    diff: Vec<Rect>,
}

impl Local {
    fn empty() -> Local {
        Local {
            actual: Vec::new(),
            diff: Vec::new(),
        }
    }
    fn stdin(N: usize, T: usize) -> Local {
        input_interactive! {
            act: [(i32, i32); N],
            diff: [(i32, i32); T],
        }
        Local {
            actual: act.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
            diff: diff.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
        }
    }
}

#[derive(Clone)]
struct Input {
    N: usize,
    T: usize,
    sigma: i32,
    rects: Vec<Rect>,
    local: Local,
}


impl Input {
    fn stdin(is_local: bool) -> Input {
        input_interactive! {
            N: usize,
            T: usize,
            sigma: i32,
            rect: [(i32, i32); N],
        }
        Input {
            N,
            T,
            sigma,
            rects: rect.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
            local: if is_local { Local::stdin(N, T) } else { Local::empty() },
        }
    }
}

use itertools::Itertools;
use proconio::{input, input_interactive};
use rand::prelude::*;
use rand_distr::Standard;
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
        #[cfg(feature = "local")]
        {
            ms - STIME
        }
    }
}
