#![allow(non_snake_case, dead_code)]

const N: usize = 20;

fn main() {
    get_time();
    let input = Input::stdin();
    let mut mt = Mt::new(768);

    let mut score = (std::f64::MIN, vec![vec![0]]);

    let (S, W) = input.merge();
    while get_time() < 2.990 {
        let board = greedy(&S, &W, input.M, &mut mt);
        let now_score = input.score(&board);
        let now = (now_score, board);
        if score.0 < now.0 {
            score = now;
        }
    }

    let (score, board) = score;
    eprintln!("{}", score as u64);

    let chars = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
    for x in 0..N {
        for y in 0..N {
            print!("{}", if board[x][y] == !0 { '.' } else { chars[board[x][y]] });
        }
        println!();
    }
}

fn greedy(S: &Vec<Vec<usize>>, W: &Vec<usize>, M: usize, mt: &mut Mt) -> Vec<Vec<usize>> {
    let mut board = vec![vec![!0; N]; N];

    for i in 0..S.len() {
        let mut xsame = (std::i64::MIN / 2, !0, !0);
        let mut ysame = (std::i64::MIN / 2, !0, !0);
        for x in 0..N {
            for y in 0..N {
                let mut xcnt = 0i64;
                let mut ycnt = 0i64;
                for k in 0..S[i].len() {
                    let v = board[x][(y + k) % N];
                    if v == S[i][k] {
                        xcnt += 1;
                    }
                    else if v != !0 {
                        xcnt = std::i64::MIN;
                    }
                    let v = board[(x + k) % N][y];
                    if v == S[i][k] {
                        ycnt += 1;
                    }
                    else if v != !0 {
                        ycnt = std::i64::MIN;
                    }
                }
                xsame = xsame.max((xcnt, x, y));
                ysame = ysame.max((ycnt, x, y));
            }
        }

        if xsame.0 < 0 && ysame.0 < 0 {
            continue;
        }
        if ysame.0 < 0 || xsame.0 > ysame.0 || (xsame.0 == ysame.0 && mt.gen_bool(0.5)) {
            let (_, x, y) = xsame;
            for k in 0..S[i].len() {
                board[x][(y + k) % N] = S[i][k];
            }
        }
        else {
            let (_, x, y) = ysame;
            for k in 0..S[i].len() {
                board[(x + k) % N][y] = S[i][k];
            }
        }
    }
    board
}

struct Input {
    M: usize,
    S: Vec<Vec<usize>>,
}

impl Input {
    fn stdin() -> Input {
        use proconio::marker::Chars;
        input! {
            n: usize,
            M: usize,
            S: [Chars; M],
        }
        let mut S = S.into_iter()
            .map(|cs| cs.into_iter().map(|c| (c as u8 - 'A' as u8) as usize).collect_vec())
            .collect_vec();
        S.sort_by(|x, y| y.len().cmp(&x.len()));
        Input {
            M, S
        }
    }

    fn score(&self, board: &Vec<Vec<usize>>) -> f64 {
        let mut dot_cnt = 0;
        let mut place_cnt = 0;
        for x in 0..N {
            for y in 0..N {
                dot_cnt += (board[x][y] == !0) as usize;
            }
        }
        for i in 0..self.M {
            'outer: for x in 0..N {
                for y in 0..N {
                    let mut xok = true;
                    let mut yok = true;
                    for k in 0..self.S[i].len() {
                        let v = board[x][(y + k) % N];
                        if v != self.S[i][k] {
                            xok = false;
                        }
                        let v = board[(x + k) % N][y];
                        if v != self.S[i][k] {
                            yok = false;
                        }
                    }
                    if xok || yok {
                        place_cnt += 1;
                        break 'outer;
                    }
                }
            }
        }
        //eprintln!("place_cnt {}", place_cnt);
        //eprintln!("dot_cnt {}", dot_cnt);

        let score =
            if place_cnt == self.M {
                1e8 * 2.0 * N as f64 * N as f64 / (2.0 * N as f64 * N as f64 - dot_cnt as f64)
            }
            else {
                1e8 * place_cnt as f64 / self.M as f64
            };
        score
    }

    fn merge(&self) -> (Vec<Vec<usize>>, Vec<usize>) {
        let mut S = self.S.clone();
        let mut W = vec![1; S.len()];
        for i in (0..S.len()).rev() {
            for j in 0..i {
                let mut ok = false;
                for k in 0..S[j].len() - S[i].len() + 1 {
                    if S[j][k..k+S[i].len()] == S[i] {
                        ok = true;
                        break;
                    }
                }
                if ok {
                    W[j] += W[i];
                    S.swap_remove(i);
                    W.swap_remove(i);
                    break;
                }
            }
        }
        while get_time() < 1.0 {
            let mut max_merge = (0, !0, !0);
            for i in 0..S.len() {
                for j in 0..S.len() {
                    if i == j {
                        continue;
                    }
                    for k in (1..S[i].len().min(S[j].len())).rev() {
                        if S[i][S[i].len() - k..S[i].len()] == S[j][0..k] && S[i].len() + S[j].len() - k <= 20 {
                            max_merge = max_merge.max((k, i, j));
                            break;
                        }
                    }
                }
            }
            if max_merge.0 > 0 {
                let (k, i, j) = max_merge;
                eprintln!("{:?} {:?} {}", S[i], S[j], k);
                S[i] = [&S[i][..], &S[j][k..]].concat();
                W[i] = W[i] + W[j];
                S.swap_remove(j);
                W.swap_remove(j);
            }
            else {
                break;
            }
        }
        /*
        let mut ins = vec![vec![]; S.len()];
        let mut out = vec![vec![]; S.len()];
        for i in 0..S.len() {
            for j in 0..S.len() {
                if i == j {
                    continue;
                }
                for k in (1..S[i].len().min(S[j].len())).rev() {
                    if S[i][S[i].len() - k..S[i].len()] == S[j][0..k] {
                        ins[j].push((k, i));
                        out[i].push((k, j));
                        break;
                    }
                }
            }
        }
        for i in 0..S.len() {
            ins[i].sort();
            ins[i].reverse();
            out[i].sort();
            out[i].reverse();
        }
        let mut used = vec![false; S.len()];
        let mut new_S = vec![];
        let mut new_W = vec![];
        for i in 0..S.len() {
            if used[i] { continue; }
            used[i] = true;
            let mut left = i;
            let mut right = i;
            let mut s = S[i].clone();
            let mut w = W[i];
            loop {
                while ins[left].len() > 0 && used[ins[left].last().unwrap().1] {
                    ins[left].pop();
                }
                while out[right].len() > 0 && used[out[right].last().unwrap().1] {
                    out[right].pop();
                }
                if ins[left].len() == 0 && out[right].len() == 0 {
                    break;
                }
                if ins[left].len() == 0 || (out[right].len() > 0 && ins[left].last().unwrap().0 < out[right].last().unwrap().0) {
                    // append right
                    let (k, next_right) = out[right].pop().unwrap();
                    let t = [&s[..], &S[next_right][k..S[next_right].len()]].concat();
                    if t.len() > 10 {
                        break;
                    }
                    used[next_right] = true;
                    right = next_right;
                    s = t;
                    w += W[next_right];
                }
                else {
                    // append left
                    let (k, next_left) = ins[left].pop().unwrap();
                    let t = [&S[next_left][0..S[next_left].len() - k], &s[..]].concat();
                    if t.len() > 10 {
                        break;
                    }
                    used[next_left] = true;
                    left = next_left;
                    s = t;
                    w += W[next_left];
                }
            }
            eprintln!("{} {} {:?} {}", left, right, s, w);
            new_S.push(s);
            new_W.push(w);
        }
        S = new_S;
        W = new_W;
        */
        let mut idx = (0..S.len()).collect_vec();
        idx.sort_by(|x, y| 
            if W[*x] == W[*y] {
                S[*y].len().cmp(&S[*x].len())
            }
            else {
                W[*y].cmp(&W[*x])
            }
        );
        (
            idx.iter().map(|i| S[*i].clone()).collect_vec(),
            idx.iter().map(|i| W[*i].clone()).collect_vec()
        )
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
