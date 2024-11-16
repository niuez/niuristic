#![allow(non_snake_case, dead_code)]

const N: usize = 20;

fn main() {
    get_time();
    let input = Input::stdin();
    let ans = dijkstra(&input);

    let mut state = State::init(ans);
    let mut mt = Mt::new(768);

    let T0 = 300000.0f64;
    let T1 = 500.0f64;
    let mut T = T0;
    let mut iter = 0;
    let TL = 1.995;
    let mut max_state = state.clone();
    loop {
        let time = get_time();
        if time >= TL {
            break;
        }
        iter += 1;
        if iter % 10 == 0 {
            let p = time / TL;
            T = T0.powf(1.0 - p) * T1.powf(p);
            //eprintln!("{} {}", T, state.score);
        }
        state.optimize(&input, &mut mt, T);
        if max_state.score < state.score {
            max_state = state.clone();
        }
    }
    state = max_state;
    let ans = state.perm;
    eprintln!("score\t{}", state.score);
    for x in ans {
        for k in 0..1 {
            print!("{}", DIRS[x]);
        }
    }
    println!();
}

#[derive(Clone)]
struct State {
    perm: Vec<usize>,
    score: f64,
}

impl State {
    fn init(perm: Vec<usize>) -> State {
        State {
            perm,
            score: 0.0,
        }
    }

    fn optimize(&mut self, input: &Input, mt: &mut Mt, T: f64) {
        //eprintln!("start optimize {}", self.perm.len());
        let mut back = vec![vec![0.0; N*N]; self.perm.len() + 1];
        let mut reach = vec![vec![0.0; N*N]; self.perm.len() + 1];
        back[self.perm.len()][input.T] = 401.0 - self.perm.len() as f64;
        reach[self.perm.len()][input.T] = 1.0;
        for t in (0..self.perm.len()).rev() {
            for i in 0..N*N {
                if i != input.T {
                    back[t][i] = back[t + 1][i] * input.P + back[t + 1][input.G[i][self.perm[t]]] * (1.0 - input.P);
                    reach[t][i] = reach[t + 1][i] * input.P + reach[t + 1][input.G[i][self.perm[t]]] * (1.0 - input.P);
                }
            }
            back[t][input.T] = 401.0 - t as f64;
            reach[t][input.T] = 1.0;
        }

        let mut dp = vec![0.0; N*N];
        dp[input.S] = 1.0;
        let mut next = vec![0.0; N*N];
        let mut next_perm = vec![];
        let mut now_score = back[0][input.S] * 250000.0;
        let mut now_goaled = 0.0;
        let mut add = 0.0;
        let mut t = 0;
        while t < self.perm.len() {
            next.fill(0.0);
            /*
            if (now_score - calc_score(input, &[&next_perm, &self.perm[t..]].concat())).abs() > 1e-5 {
                assert!(false);
            }
            */
            let mut query = mt.gen_range(0..4);
            if next_perm.len() + self.perm.len() - t + 1 > 200 {
                query = mt.gen_range(2..4);
            }
            if query == 0 && next_perm.len() + self.perm.len() - t + 1 <= 200 {
                // continue same dir
                let mut next_score = now_goaled;
                for i in 0..N*N {
                    next[i] += dp[i] * input.P;
                    next[input.G[i][self.perm[t]]] += dp[i] * (1.0 - input.P);
                }
                for i in 0..N*N {
                    next_score += next[i] * (back[t][i] - (add + 1.0) * reach[t][i]);
                }
                next_score *= 250000.0;
                /*
                if dbg!((next_score - back_calc_score(input, &[&next_perm, &vec![self.perm[t]], &self.perm[t..]].concat())).abs()) > 1e-5 {
                    assert!(false);
                }
                */
                if next_score >= now_score || mt.gen_bool(((next_score - now_score) / T).exp()) {
                    now_score = next_score;
                    next_perm.push(self.perm[t]);
                    add += 1.0;
                    std::mem::swap(&mut next, &mut dp);
                    now_goaled += dp[input.T] * (401.0 - next_perm.len() as f64);
                    dp[input.T] = 0.0;
                    continue;
                }
            }
            else if query == 1 && next_perm.len() + self.perm.len() - t + 1 <= 200 {
                // add random dir
                let next_dir = mt.gen_range(0..4);
                let mut next_score = now_goaled;

                for i in 0..N*N {
                    next[i] += dp[i] * input.P;
                    next[input.G[i][next_dir]] += dp[i] * (1.0 - input.P);
                }
                for i in 0..N*N {
                    next_score += next[i] * (back[t][i] - (add + 1.0) * reach[t][i]);
                }
                next_score *= 250000.0;
                if next_score >= now_score || mt.gen_bool(((next_score - now_score) / T).exp()) {
                    now_score = next_score;
                    next_perm.push(next_dir);
                    add += 1.0;
                    std::mem::swap(&mut next, &mut dp);
                    now_goaled += dp[input.T] * (401.0 - next_perm.len() as f64);
                    dp[input.T] = 0.0;
                    continue;
                }
            }
            else if query == 2 {
                // delete next dir
                let mut next_score = now_goaled;
                for i in 0..N*N {
                    next_score += dp[i] * (back[t + 1][i] - (add - 1.0) * reach[t + 1][i]);
                }
                next_score *= 250000.0;
                if next_score >= now_score || mt.gen_bool(((next_score - now_score) / T).exp()) {
                    now_score = next_score;
                    add -= 1.0;
                    t += 1;
                    continue;
                }
            }
            else if query == 3 {
                // replace next dir
                let next_dir = mt.gen_range(0..3);
                let next_dir = if self.perm[t] <= next_dir { next_dir + 1 } else { next_dir };
                let mut next_score = now_goaled;
                for i in 0..N*N {
                    next[i] += dp[i] * input.P;
                    next[input.G[i][next_dir]] += dp[i] * (1.0 - input.P);
                }
                for i in 0..N*N {
                    next_score += next[i] * (back[t + 1][i] - (add) * reach[t + 1][i]);
                }
                next_score *= 250000.0;
                if next_score >= now_score || mt.gen_bool(((next_score - now_score) / T).exp()) {
                    now_score = next_score;
                    next_perm.push(next_dir);
                    std::mem::swap(&mut next, &mut dp);
                    now_goaled += dp[input.T] * (401.0 - next_perm.len() as f64);
                    dp[input.T] = 0.0;
                    t += 1;
                    continue;
                }
            }

            // if no accept, do default
            next.fill(0.0);
            for i in 0..N*N {
                next[i] += dp[i] * input.P;
                next[input.G[i][self.perm[t]]] += dp[i] * (1.0 - input.P);
            }
            next_perm.push(self.perm[t]);
            std::mem::swap(&mut next, &mut dp);
            now_goaled += dp[input.T] * (401.0 - next_perm.len() as f64);
            dp[input.T] = 0.0;
            t += 1
        }

        /*
        while next_perm.len() + 1 <= 200 {
            // add random dir
            let next_dir = mt.gen_range(0..4);
            for i in 0..N*N {
                next[i] = dp[i] * input.P + dp[input.R[i][next_dir]] * (1.0 - input.P);
            }
            let next_score = now_score + 250000.0 * next[input.T] * (400.0 - next_perm.len() as f64);
            if next_score >= now_score || mt.gen_bool(((next_score - now_score) / T).exp()) {
                now_score = next_score;
                next_perm.push(next_dir);
                std::mem::swap(&mut next, &mut dp);
                dp[input.T] = 0.0;
                continue;
            }
            else {
                break;
            }
        }
        */
        self.perm = next_perm;
        self.score = now_score;
    }

}

fn back_calc_score(input: &Input, ans: &Vec<usize>) -> f64 {
    let mut back = vec![vec![0.0; N*N]; ans.len() + 1];
    let mut reach = vec![vec![0.0; N*N]; ans.len() + 1];
    back[ans.len()][input.T] = 401.0 - ans.len() as f64;
    reach[ans.len()][input.T] = 1.0;
    for t in (0..ans.len()).rev() {
        for i in 0..N*N {
            if i != input.T {
                back[t][i] = back[t + 1][i] * input.P + back[t + 1][input.G[i][ans[t]]] * (1.0 - input.P);
                reach[t][i] = reach[t + 1][i] * input.P + reach[t + 1][input.G[i][ans[t]]] * (1.0 - input.P);
            }
        }
        back[t][input.T] = 401.0 - t as f64;
        reach[t][input.T] = 1.0;
        /*
        if t == 1 {
            eprintln!("evaluate {} {}", back[t][22], back[t][23]);
            eprintln!("reach {} {}", reach[t][22], reach[t][23]);
        }
        */
    }
    back[0][input.S] * 250000.0
}

fn calc_score(input: &Input, ans: &Vec<usize>) -> f64 {
    let mut dp = vec![0.0; N*N];
    dp[input.S] = 1.0;
    let mut next = vec![0.0; N*N];
    let mut score = 0.0;
    for t in 0..ans.len() {
        next.fill(0.0);
        for i in 0..N*N {
            next[i] += dp[i] * input.P;
            next[input.G[i][ans[t]]] += dp[i] * (1.0 - input.P);
        }
        std::mem::swap(&mut next, &mut dp);
        score += dp[input.T] * (400.0 - t as f64) * 250000.0;
        dp[input.T] = 0.0;
    }
    score
}

fn dijkstra(input: &Input) -> Vec<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut dist = vec![(std::usize::MAX, !0, !0); N*N];
    let mut que = BinaryHeap::new();
    dist[input.S] = (0, !0, !0);
    que.push(Reverse((dist[input.S].0, input.S)));

    while let Some(Reverse((d, v))) = que.pop() {
        if d > dist[v].0 {
            continue;
        }
        for dir in 0..4 {
            let cost = 1;
            let u = input.G[v][dir];
            if d + cost < dist[u].0 {
                dist[u] = (d + cost, dir, v);
                que.push(Reverse((dist[u].0, u)));
            }
        }
    }
    let mut before = input.T;
    let mut ans = vec![];
    loop {
        let x = dist[before].1;
        if x == !0 {
            break;
        }
        ans.push(x);
        before = dist[before].2;
    }
    ans.reverse();
    eprintln!("{} {:?}", ans.len(), ans);
    ans
}

const DIRS: [char; 4] = ['R', 'L', 'D', 'U'];

struct Input {
    S: usize,
    T: usize,
    P: f64,
    G: Vec<Vec<usize>>,
}

impl Input {
    fn stdin() -> Self {
        use proconio::marker::Chars;
        input! {
            sx: usize,
            sy: usize,
            tx: usize,
            ty: usize,
            P: f64,
            h: [Chars; N], // N * N-1
            v: [Chars; N - 1], // N-1 * N
        }

        let mut G = (0..N*N).map(|i| vec![i, i, i, i]).collect_vec();
        for i in 0..N {
            for j in 0..N {
                let x = i * N + j;
                if j + 1 < N && h[i][j] == '0' {
                    G[x][0] = x + 1;
                }
                if j > 0 && h[i][j - 1] == '0' {
                    G[x][1] = x - 1;
                }
                if i + 1 < N && v[i][j] == '0' {
                    G[x][2] = x + N;
                }
                if i > 0 && v[i - 1][j] == '0' {
                    G[x][3] = x - N;
                }
            }
        }
        Input {
            S: sx * N + sy,
            T: tx * N + ty,
            P,
            G,
        }
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
