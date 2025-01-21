#![allow(non_snake_case, dead_code)]

const SERVER: u8 = 8;
const ROAD: u8 = 9;

fn main() {
    get_time();
    let input = Input::from_stdin();
    let Input { N, K, mut C } = input.clone();
    C[get_start(&input, 0)] = SERVER;

    let mut moves = vec![];
    let mut connect_cnt = 0;

    loop {
        let mut next_moves = vec![];
    }
}

fn calculate_next_moves(N: usize, C: &Vec<u8>) -> Vec<(usize, usize)> {
    let INF = std::usize::MAX / 2;
    let mut dist = vec![(INF, !0); N * N];
    for i in 0..N {
        for j in 0..N {
            if C[i * N + j] == SERVER {
                dist[i * N + j] = (0, !0);
                {
                    let mut now = 0;
                    for k in (0..j).rev() {
                        let v = i * N + k;
                        now =
                            if C[v] == !0 { now + 0 }
                            else if C[v] == ROAD { now + 0 }
                            else { now + 10 };
                        dist[v] = dist[v].min((now, v + 1));
                    }
                }
                {
                    let mut now = 0;
                    for k in (j + 1)..N {
                        let v = i * N + k;
                        now =
                            if C[v] == !0 { now + 0 }
                            else if C[v] == ROAD { now + 0 }
                            else { now + 10 };
                        dist[v] = dist[v].min((now, v - 1));
                    }
                }
                break;
            }
        }
    }
    for j in 0..N {
        for i in 0..N {
            if C[i * N + j] == SERVER {
                dist[i * N + j] = (0, !0);
                {
                    let mut now = 0;
                    for k in (0..i).rev() {
                        let v = k * N + j;
                        now =
                            if C[v] == !0 { now + 0 }
                            else if C[v] == ROAD { now + 0 }
                            else { now + 10 };
                        dist[v] = dist[v].min((now, v + N));
                    }
                }
                {
                    let mut now = 0;
                    for k in (i+1)..N {
                        let v = k * N + j;
                        now =
                            if C[v] == !0 { now + 0 }
                            else if C[v] == ROAD { now + 0 }
                            else { now + 10 };
                        dist[v] = dist[v].min((now, v - N));
                    }
                }
                break;
            }
        }
    }

    let mut que = BinaryHeap::new();
    for i in 0..N*N {
        if dist[i].0 == 0 {
            que.push(Reverse((dist[i], i)));
        }
    }

    while let Some(Reverse((d, v))) = que.pop() {
        if d > dist[v] { continue; }
    }
    unimplemented!();
}

fn get_start(input: &Input, V: u8) -> usize {
    let mut min = (std::usize::MAX, !0);
    for i in 0..input.N {
        for j in 0..input.N {
            if input.C[i * input.N + j] == V {
                min = min.min((i.abs_diff(input.N / 2) + j.abs_diff(input.N / 2), i * input.N + j));
            }
        }
    }
    min.1
}

#[derive(Clone)]
struct Input {
    N: usize,
    K: usize,
    C: Vec<u8>,
}

impl Input {
    fn from_stdin() -> Input {
        input! {
            N: usize,
            K: usize,
            S: [Chars; N],
        }

        Input {
            N,
            K,
            C: (0..N*N).map(|i| (S[i / N][i % N].to_digit(10).unwrap() as u8).wrapping_sub(1)).collect_vec(),
        }
    }
}


use std::{cmp::Reverse, collections::BinaryHeap};

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
