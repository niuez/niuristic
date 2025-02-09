#![allow(non_snake_case, dead_code)]

fn main() {
    get_time();

    let input = Input::from_stdin();
}

const N: usize = 9;
const M: usize = 20;
const K: usize = 81;
const MOD: u32 = 998244353;

const PUSH: [usize; 9] = [0, 1, 2, 0 + N, 1 + N, 2 + N, 0 + 2 * N, 1 + 2 * N, 2 + 2 * N];

struct Input {
    A: [u32; N*N],
    S: [[u32; 9]; M],
}

impl Input {
    fn from_stdin() -> Input {
        input! {
            A: [u32; N*N],
            S: [[u32; 9]; M],
        }

        Input {
            A: A.try_into().unwrap(),
            S: S.into_iter().map(|x| x.try_into().unwrap()).collect_vec().try_into().unwrap(),
        }
    }
}


use itertools::Itertools;
use proconio::{input, input_interactive, marker::Chars};
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
