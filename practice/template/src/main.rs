#![allow(non_snake_case, dead_code)]

fn main() {
    get_time();
    let N = 50;
    let C = 10_000_000;

    let start = get_time();
    let simple = simple_sampling(768, N, C);
    let simple_time = get_time() - start;
    let gibbs = fast_gibbs_sampling(768, N, C);
    let gibbs_time = get_time() - simple_time;
    for i in 0..N {
        eprintln!("{}\t{}\t{}", simple[i] - gibbs[i], simple[i], gibbs[i]);
    }
    eprintln!("{} {}", simple_time, gibbs_time);
}

fn simple_sampling(seed: u128, n: usize, cnt: usize) -> Vec<f64> {
    let mut avg = vec![0.0; n];
    let mut mt = Mt::new(seed);
    let dist = rand_distr::Exp::new(1e-5).unwrap();
    for c in 0..cnt {
        let mut v = vec![];
        while v.len() < n {
            let x = dist.sample(&mut mt) as f64;
            if x < 1e6 {
                v.push(x);
            }
        }
        v.sort_by(|x, y| x.partial_cmp(y).unwrap());
        for i in 0..n {
            avg[i] = avg[i] + v[i];
        }
    }
    for i in 0..n {
        avg[i] /= cnt as f64;
    }
    avg
}

fn gibbs_sampling(seed: u128, n: usize, cnt: usize) -> Vec<f64> {
    let mut avg = vec![0.0; n];
    let mut v = simple_sampling(seed, n, 1);
    let mut mt = Mt::new(seed);
    let cum = |x: f64| 1.0 - (-1e-5 * x).exp();
    for c in 0..cnt {
        for i in 0..n {
            let lower = if i == 0 { 0.0 } else { cum(v[i - 1]) };
            let upper = if i + 1 == n { cum(1e6) } else { cum(v[i + 1]) };
            v[i] = - (1.0 - mt.gen_range(lower..upper)).ln() / 1e-5;
        }

        for i in 0..n {
            avg[i] = avg[i] + v[i];
        }
    }

    for i in 0..n {
        avg[i] /= cnt as f64;
    }
    avg
}

fn fast_gibbs_sampling(seed: u128, n: usize, cnt: usize) -> Vec<f64> {
    let mut avg = vec![0.0; n];
    let mut v = simple_sampling(seed, n, 1);
    let mut mt = Mt::new(seed);
    let cum = |x: f64| 1.0 - (-1e-5 * x).exp();
    v.push(1e6);
    let mut cums = (0..n+1).map(|i| cum(v[i])).collect_vec();
    for c in 0..cnt {
        for i in 0..n {
            let lower = if i == 0 { 0.0 } else { cums[i - 1] };
            let upper = cums[i + 1];
            cums[i] = mt.gen_range(lower..upper);
            v[i] = - (1.0 - cums[i]).ln() / 1e-5;
        }

        for i in 0..n {
            avg[i] = avg[i] + v[i];
        }
    }

    for i in 0..n {
        avg[i] /= cnt as f64;
    }
    avg
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
