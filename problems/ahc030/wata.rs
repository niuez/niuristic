#![allow(non_snake_case)]

use bitset_fixed::BitSet;
use itertools::Itertools;
use proconio::input_interactive;
use rand::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

const DIJ: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

/*
解法概略

以下のステップを繰り返すことで、占いとその結果に基づく配置候補の更新を行う。採掘は一切行わない。
1. 焼き鈍し法によりこれまでの占い履歴に対する尤度が高い配置を沢山求め、候補に追加する。尤度の低い候補は削除する。
2. 得られた配置候補に限定することで、占い結果と真の配置との間の相互情報量を近似的に計算し、相互情報量/コストが出来るだけ大きいマス集合に対して占いを行う。
3. 占い結果を反映させて、各候補の事後確率を更新する。

各配置は一様ランダムに生成されるので、これまでの占い履歴 Q に対する各配置 x の事後確率 P(x|Q) は尤度 P(Q|x) に比例する。
M=2などの場合を除くと、全ての配置を全列挙することは出来ないため、焼き鈍し法により尤度が高い配置を重点的に列挙する。
焼き鈍し法が見つけた配置の集合を X' とし、可能な配置を X' に絞って考えることで、事後確率を近似的に計算できる。
ただし、焼き鈍しが尤度最大の配置を見つけることに失敗していた場合には、大幅に間違った値になる可能性があるため、焼き鈍しの性能を上げることが非常に重要である。

ある配置の事後確率がほぼ1であれば、自信を持ってその配置を答えることが出来る。
確信出来るようになるまでに残りどれだけの情報が必要かの尺度として、エントロピー H(X) := Σ -P(x) log P(x) を用いることにする。
エントロピーがほぼ0になれば終了なので、占い結果を得た後のエントロピーが出来るだけ小さくなり、かつコストが出来るだけ小さいマス集合に対して占いを行いたい。

真の配置が x であるときに、マス集合 S に対する占いの結果が i となる確率 P(i|x) が計算出来る。
真の配置が x である確率は P(x) なので、占い結果が i となる確率は P(i) = Σ P(i|x)P(x) である。
占い結果 i を知ると、各配置の事後確率が P(x) から P(x|S,i) に変化し、エントロピーが減少する。
この減少量の期待値は「相互情報量」と呼ばれる値で、
I(X;S) := Σ P(i,x) log(P(i,x) / P(i)P(x)) = Σ P(i|x)P(x) log(P(i|x) / P(i))
という式で計算できる。
相互情報量/コスト を評価関数とし、1マス選んでSに追加・除去する操作を近傍とした山登り法を行うことで、占うマス集合 S を構築する。
*/
fn main() {
    get_time();
    let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(189048320);
    let input = read_input();
    // 時間調整のために難易度(=必要ターン数)を推定する。可能な配置数が多く、epsが大きいほど難易度が高い。
    let num_possibilities = (0..input.m)
        .map(|p| ((input.n - input.max_i[p]) * (input.n - input.max_j[p])) as f64)
        .product::<f64>();
    let capacity = 1.0 + input.eps * input.eps.log2() + (1.0 - input.eps) * (1.0 - input.eps).log2();
    let difficulty = num_possibilities.log2() / capacity;
    eprintln!("!log dif {:.3}", difficulty);
    // 2つのポリオミノの位置を入れ替える操作を行うために、入れ替えた際にどれだけ位置をずらせばよいかを予め計算しておく
    // swaps[p][q] := pとq+Δが出来るだけ一致するようなΔ
    let mut swaps = mat![vec![]; input.m; input.m];
    for p in 0..input.m {
        let mut bs = vec![false; input.n2];
        for &ij in &input.ts[p] {
            bs[ij] = true;
        }
        for q in 0..input.m {
            if p == q {
                continue;
            }
            let mut list = vec![];
            for di in -(input.max_i[q] as i32)..=input.max_i[p] as i32 {
                for dj in -(input.max_j[q] as i32)..=input.max_j[p] as i32 {
                    let mut count = 0;
                    for &ij in &input.ts[q] {
                        let i = ij / input.n + di as usize;
                        let j = ij % input.n + dj as usize;
                        if i < input.n && j < input.n && bs[i * input.n + j] {
                            count += 1;
                        }
                    }
                    list.push((count, (di as usize, dj as usize)));
                }
            }
            list.sort();
            list.reverse();
            list.truncate(4);
            swaps[p][q] = list.into_iter().map(|(_, ij)| ij).collect_vec();
        }
    }
    let mut sim = Sim::new(&input);
    let mut state = State::new(&input);
    // 尤度が高い配置候補を覚えておき、更新していく
    let mut pool: Vec<Entry> = vec![];
    let ITER = ((30000.0 * (160.0 / difficulty).powi(2)).round() as usize).min(5000000);
    for t in 0.. {
        println!("# time = {:.3}", get_time());
        if sim.rem == 0 {
            eprintln!("!log giveup 1 {} {}", input.n, input.m);
            break;
        }
        if get_time() > 2.9 {
            sim.giveup();
            break;
        }
        // 各配置の対数尤度を更新する
        for e in &mut pool {
            if e.count.is_empty() && !sim.failed.is_empty() {
                e.count = input.get_count(&e.ps);
            }
            e.prob = sim.ln_prob(&state, &e.count, &e.ps);
        }
        pool.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());
        let mut set = FxHashMap::default();
        for e in &pool {
            set.insert(e.hash, e.prob);
        }
        if t == 0 {
            // 1ターン目は全ての配置が等確率なので、ランダムに候補を生成する
            for _ in 0..ITER {
                for p in 0..input.m {
                    let i = rng.gen_range(0..input.n - input.max_i[p]);
                    let j = rng.gen_range(0..input.n - input.max_j[p]);
                    state.move_to(p, i * input.n + j);
                }
                if set.insert(state.hash, 0.0).is_none() {
                    pool.push(Entry {
                        hash: state.hash,
                        prob: 0.0,
                        ps: state.ps.clone(),
                        count: state.count.clone(),
                    });
                }
            }
        } else {
            // 一番尤度の高い配置からスタートし、焼き鈍し法を実行することで配置候補を沢山生成する
            let mut crt = pool[0].prob;
            for p in 0..input.m {
                state.move_to(p, pool[0].ps[p]);
            }
            print!("# {:.3} -> ", crt);
            let mut max = crt;
            const T0: f64 = 2.0;
            const T1: f64 = 1.0;
            // TLEにならないように残り時間が少なくなったら反復回数を減らす
            let ITER = (ITER as f64 * (3.0 - get_time()).min(1.0)) as usize;
            for t in 0..ITER {
                let temp = T0 + (T1 - T0) * t as f64 / ITER as f64;
                let coin = rng.gen_range(0..10);
                if coin < 3 {
                    // ポリオミノをランダムに選び、上下左右に1マス移動
                    let p = rng.gen_range(0..input.m);
                    let (di, dj) = DIJ[rng.gen_range(0..4)];
                    let i2 = state.ps[p] / input.n + di;
                    let j2 = state.ps[p] % input.n + dj;
                    if i2 < input.n - input.max_i[p] && j2 < input.n - input.max_j[p] {
                        let bk = state.ps[p];
                        state.move_to(p, i2 * input.n + j2);
                        let next = if let Some(next) = set.get(&state.hash) {
                            *next
                        } else {
                            let next = sim.ln_prob_state(&state);
                            set.insert(state.hash, next);
                            if next - max >= -10.0 {
                                pool.push(Entry {
                                    hash: state.hash,
                                    prob: next,
                                    ps: state.ps.clone(),
                                    count: state.count.clone(),
                                });
                            }
                            next
                        };
                        if crt <= next || rng.gen_bool(((next - crt) / temp).exp()) {
                            crt = next;
                        } else {
                            state.move_to(p, bk);
                        }
                    }
                } else if coin < 4 {
                    // ポリオミノをランダムに選び、ランダムな位置に移動
                    let p = rng.gen_range(0..input.m);
                    let i2 = rng.gen_range(0..input.n - input.max_i[p]);
                    let j2 = rng.gen_range(0..input.n - input.max_j[p]);
                    let bk = state.ps[p];
                    state.move_to(p, i2 * input.n + j2);
                    let next = if let Some(next) = set.get(&state.hash) {
                        *next
                    } else {
                        let next = sim.ln_prob_state(&state);
                        set.insert(state.hash, next);
                        if next - max >= -10.0 {
                            pool.push(Entry {
                                hash: state.hash,
                                prob: next,
                                ps: state.ps.clone(),
                                count: state.count.clone(),
                            });
                        }
                        next
                    };
                    if crt <= next || rng.gen_bool(((next - crt) / temp).exp()) {
                        crt = next;
                    } else {
                        state.move_to(p, bk);
                    }
                } else {
                    // ポリオミノを2つランダムに選び、互いの位置を交換
                    let p = rng.gen_range(0..input.m);
                    let q = rng.gen_range(0..input.m);
                    if p == q {
                        continue;
                    }
                    let (pi, pj) = (state.ps[p] / input.n, state.ps[p] % input.n);
                    let (qi, qj) = (state.ps[q] / input.n, state.ps[q] % input.n);
                    let &dp = swaps[q][p].choose(&mut rng).unwrap();
                    let &dq = swaps[p][q].choose(&mut rng).unwrap();
                    let pi2 = qi + dp.0;
                    let pj2 = qj + dp.1;
                    if pi2 >= input.n - input.max_i[p] || pj2 >= input.n - input.max_j[p] {
                        continue;
                    }
                    let qi2 = pi + dq.0;
                    let qj2 = pj + dq.1;
                    if qi2 >= input.n - input.max_i[q] || qj2 >= input.n - input.max_j[q] {
                        continue;
                    }
                    state.move_to(p, pi2 * input.n + pj2);
                    state.move_to(q, qi2 * input.n + qj2);
                    let next = if let Some(next) = set.get(&state.hash) {
                        *next
                    } else {
                        let next = sim.ln_prob_state(&state);
                        set.insert(state.hash, next);
                        if next - max >= -10.0 {
                            pool.push(Entry {
                                hash: state.hash,
                                prob: next,
                                ps: state.ps.clone(),
                                count: state.count.clone(),
                            });
                        }
                        next
                    };
                    if crt <= next || rng.gen_bool(((next - crt) / temp).exp()) {
                        crt = next;
                    } else {
                        state.move_to(p, pi * input.n + pj);
                        state.move_to(q, qi * input.n + qj);
                    }
                }
                max.setmax(crt);
            }
            println!("{:.3}", max);
            pool.sort_by(|a, b| b.prob.partial_cmp(&a.prob).unwrap());
        }
        // 得られた候補集合に対して、対数尤度→事後確率の変換を行う
        let max = pool[0].prob;
        let mut sum = 0.0;
        for e in &mut pool {
            e.prob = (e.prob - max).exp();
            sum += e.prob;
        }
        for e in &mut pool {
            e.prob /= sum;
        }
        while pool.len() > 1 && pool[pool.len() - 1].prob < 1e-9 {
            pool.pop();
        }
        println!("# pool = {}", pool.len());
        #[cfg(feature = "local")]
        {
            // testerを改造して真の解も受け取れるようにし、真の解の尤度と比較することで、運が悪かったのか焼き鈍しに失敗しているのかを確認している
            for i in 0..input.m {
                state.move_to(i, input.ps[i]);
            }
            println!(
                "# {:04x}: {:.5}",
                state.hash & 0xffff,
                (sim.ln_prob_state(&state) - max).exp() / sum
            );
        }
        for i in 0..pool.len().min(10) {
            println!("# {:04x}: {:.5}", pool[i].hash & 0xffff, pool[i].prob);
        }
        // 尤度が一番高い配置を推測結果として出力する場合の評価値を計算
        let T = input.get_positives(&pool[0].ps);
        let mut pT = 0.0;
        let mut info_T = 0.0;
        for e in &pool {
            if e.prob < 1e-3 || T != input.get_positives(&e.ps) {
                pT += e.prob;
            } else {
                info_T -= e.prob * e.prob.ln();
            }
        }
        if pT == 0.0 {
            info_T = 1e10;
        } else {
            info_T -= pT * pT.ln();
            info_T /= pT;
        }
        println!("# info_T = {:.3}", info_T);
        let mut query_ps;
        let info_Q = {
            // 占うマス集合を決定する
            // 候補全体を使うと遅すぎるので尤度の高い上位何件かに絞る
            let mut size = pool
                .len()
                .min((100.0 * (160.0 / difficulty).powi(2) * (3.0 - get_time()).min(1.0)).max(2.0) as usize);
            while size > 2 && pool[0].prob * 1e-4 > pool[size - 1].prob {
                size -= 1;
            }
            let pool = &mut pool[..size];
            let total = pool.iter().map(|e| e.prob).sum::<f64>();
            for e in pool.iter_mut() {
                e.prob /= total;
                if e.count.is_empty() {
                    e.count = input.get_count(&e.ps);
                }
            }
            // 全ての配置候補で埋蔵量が同じマスは占っても情報が得られないので除外し、後でコスト調整用に使う
            let mut same = vec![true; input.n2];
            for e in pool.iter().skip(1) {
                for ij in 0..input.n2 {
                    same[ij] = same[ij] && e.count[ij] == pool[0].count[ij];
                }
            }
            let mut query = Query::new(&input, pool);
            // 各マスを、そのマス単体でクエリした時の相互情報量の降順にソート
            let mut ps = (0..input.n2)
                .filter(|&ij| !same[ij])
                .map(|ij| {
                    query.flip(ij);
                    let eval = query.eval(&sim, 0, 0);
                    query.flip(ij);
                    (eval, ij)
                })
                .collect_vec();
            ps.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
            let ps = ps.into_iter().map(|(_, ij)| ij).collect_vec();
            // 情報量のないマスは、埋蔵量が正のマス→0のマスの順にソート
            let mut qs = (0..input.n2).filter(|&ij| same[ij]).collect_vec();
            qs.sort_by_key(|&ij| pool[0].count[ij].min(1) as usize * 1000000 + rng.gen_range(0..1000000));
            qs.reverse();
            let mut crt = -1e100;
            let mut add_k = 0;
            let mut add_v = 0;
            for _ in 0..3 {
                // 各マスを順番に見ていき、追加・削除して相互情報量/コストが大きくなるなら採用する山登り法
                let mut change = false;
                for &ij in &ps {
                    query.flip(ij);
                    if crt.setmax(query.eval(&sim, add_k, add_v)) {
                        change = true;
                    } else {
                        query.flip(ij);
                    }
                }
                if !change {
                    break;
                }
                if difficulty <= 160.0 {
                    // 情報量0のマスを追加・削除するとこで、相互情報量/コストを最大化
                    // 難易度が高い場合、クエリ回数が足りなくなる場合があるので、余計なマスは追加しない
                    while add_k < qs.len() {
                        if crt.setmax(query.eval(&sim, add_k + 1, add_v + pool[0].count[qs[add_k]])) {
                            add_v += pool[0].count[qs[add_k]];
                            add_k += 1;
                        } else {
                            break;
                        }
                    }
                    while add_k > 0 {
                        if crt.setmax(query.eval(&sim, add_k - 1, add_v - pool[0].count[qs[add_k - 1]])) {
                            add_v -= pool[0].count[qs[add_k - 1]];
                            add_k -= 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            query_ps = query.get_ps();
            query_ps.extend(qs[..add_k].iter().cloned());
            let info_Q = crt;
            for e in pool.iter_mut() {
                e.prob *= total;
            }
            info_Q
        };
        println!("# info_Q = {}", info_Q);
        if info_T > info_Q {
            // 推測の評価値の方が高い場合、推測を行う
            // 難易度が高い場合、焼き鈍しに失敗して真の解を見落としている可能性が高く、推測に失敗することが多いので、いきなり推測を行わず、一旦それらのマスに対して占いを実行
            let T = (0..input.n2).filter(|&ij| T[ij]).collect_vec();
            if difficulty <= 160.0 || sim.query.len() > 0 && sim.query[sim.query.len() - 1].0 == T {
                if sim.ans(&T) {
                    eprintln!("!log turn {}", 2 * input.n2 - sim.rem);
                    break;
                } else if sim.failed.len() == 1 {
                    state.count = input.get_count(&state.ps);
                }
            } else {
                sim.query(&T);
                state.add_query(&input, &T);
            }
        } else {
            // 占いの評価値の方が高い場合、占いを行う
            sim.query(&query_ps);
            state.add_query(&input, &query_ps);
        }
        // 配置候補のうち、尤度が低いものを削除(本当は占い結果を反映後に削除したいが、そもそも反映に時間がかかるので反映前に消している)
        pool.truncate((100.0 * (160.0 / difficulty).powi(2) * (3.0 - get_time()).min(1.0)).max(1.0) as usize);
    }
    eprintln!("Time = {:.3}", get_time());
    eprintln!("!log miss {}", sim.failed.len());
}

/// 配置候補の情報
#[derive(Clone, Debug)]
struct Entry {
    hash: u64,
    prob: f64,
    ps: Vec<usize>,
    count: Vec<u8>,
}

/// 占いの相互情報量を差分更新するためのデータ
struct Query<'a> {
    in_query: Vec<bool>,
    count: Vec<u8>,
    k: usize,
    pool: &'a [Entry],
}

impl<'a> Query<'a> {
    fn new(input: &Input, pool: &'a [Entry]) -> Self {
        Self {
            in_query: vec![false; input.n2],
            count: vec![0; pool.len()],
            k: 0,
            pool,
        }
    }
    /// マス(i,j)を占うかどうかを反転する
    fn flip(&mut self, ij: usize) {
        if self.in_query[ij] {
            self.in_query[ij] = false;
            for (c, p) in self.count.iter_mut().zip(self.pool) {
                *c -= p.count[ij];
            }
            self.k -= 1;
        } else {
            self.in_query[ij] = true;
            for (c, p) in self.count.iter_mut().zip(self.pool) {
                *c += p.count[ij];
            }
            self.k += 1;
        }
    }
    /// 占うマス集合
    fn get_ps(&self) -> Vec<usize> {
        (0..self.in_query.len()).filter(|&ij| self.in_query[ij]).collect_vec()
    }
    /// 解xが正解の確率がp[x]、クエリの結果がyとなる確率がp[y]のとき、相互情報量は Σ p[y|x]p[x] log(p[y|x]/p[y])
    fn eval(&self, sim: &Sim, add_k: usize, add_v: u8) -> f64 {
        let k = self.k + add_k;
        // クエリ結果がrとなる確率py[r]を計算
        let mut py = vec![];
        for x in 0..self.pool.len() {
            let v = (self.count[x] + add_v) as usize;
            let lb = sim.probs_lb[k][v];
            if py.len() < lb + sim.probs[k][v].len() {
                py.resize(lb + sim.probs[k][v].len(), 0.0);
            }
            let px = self.pool[x].prob;
            for (&p, pyr) in sim.probs[k][v].iter().zip(&mut py[lb..]) {
                *pyr += p.0 * px;
            }
        }
        for pyr in &mut py {
            *pyr = pyr.ln();
        }
        // 相互情報量を計算
        // 上手く式変形するとループの内側でのlog計算を削除でき、かなり高速になる
        let mut info = 0.0;
        for x in 0..self.pool.len() {
            let px = self.pool[x].prob;
            let v = (self.count[x] + add_v) as usize;
            let lb = sim.probs_lb[k][v];
            for (&p, ln_pyr) in sim.probs[k][v].iter().zip(&py[lb..]) {
                info += p.0 * px * (p.1 - ln_pyr);
            }
        }
        info * (k as f64).sqrt()
    }
}

/// 焼き鈍しの差分計算用データ
struct State {
    ts: Vec<Vec<usize>>,
    /// 各ポリオミノの配置
    ps: Vec<usize>,
    /// 各マスの埋蔵量(失敗した占い結果と同じ領域かを判定するのに使用しており、失敗するまでは不要なので計算を省いている)
    count: Vec<u8>,
    /// これまでの各占いに対する合計埋蔵量
    query_count: Vec<u8>,
    /// [p][ij][q] := p番のポリオミノを(i,j)に配置した時の、q番の占いに対する合計埋蔵量の増加量
    pij_to_query_count: Vec<Vec<Vec<u8>>>,
    hashes: Vec<Vec<u64>>,
    hash: u64,
}

impl State {
    fn new(input: &Input) -> Self {
        let mut rng = rand_pcg::Pcg64Mcg::seed_from_u64(8904281);
        let mut hashes = mat![0; input.m; input.n2];
        for p in 0..input.m {
            // 同じ図形は同じhash値を使用することで同じ形の図形の位置を入れ替えた状態を同一視する
            // (同じ図形が同じ位置にある状態が正解の場合にhashが衝突して解が見つからなくなるが、稀なのでそのようなレアケースは捨てて速度を優先)
            if p > 0 && input.ts[p - 1] == input.ts[p] {
                hashes[p] = hashes[p - 1].clone();
            } else {
                for ij in 0..input.n2 {
                    hashes[p][ij] = rng.gen::<u64>();
                }
            }
        }
        let mut hash = 0;
        for p in 0..input.m {
            hash ^= hashes[p][0];
        }
        Self {
            ts: input.ts.clone(),
            ps: vec![0; input.m],
            count: vec![],
            query_count: vec![],
            pij_to_query_count: mat![vec![]; input.m; input.n2],
            hashes,
            hash,
        }
    }
    /// p番のポリオミノを(i,j)に移動する
    fn move_to(&mut self, p: usize, tij: usize) {
        self.hash ^= self.hashes[p][self.ps[p]] ^ self.hashes[p][tij];
        for ((&sub, &add), qc) in self.pij_to_query_count[p][self.ps[p]]
            .iter()
            .zip(&self.pij_to_query_count[p][tij])
            .zip(&mut self.query_count)
        {
            *qc += add - sub;
        }
        if !self.count.is_empty() {
            for &ij in &self.ts[p] {
                self.count[ij + self.ps[p]] -= 1;
                self.count[ij + tij] += 1;
            }
        }
        self.ps[p] = tij;
    }
    /// 占ったマス集合qsを追加
    fn add_query(&mut self, input: &Input, qs: &Vec<usize>) {
        let mut in_query = vec![false; input.n2];
        for &ij in qs {
            in_query[ij] = true;
        }
        for p in 0..input.m {
            for di in 0..input.n - input.max_i[p] {
                for dj in 0..input.n - input.max_j[p] {
                    let dij = di * input.n + dj;
                    let mut c = 0;
                    for &ij in &self.ts[p] {
                        let ij = ij + dij;
                        if in_query[ij] {
                            c += 1;
                        }
                    }
                    self.pij_to_query_count[p][dij].push(c);
                }
            }
        }
        let count = input.get_count(&self.ps);
        let mut c = 0;
        for &ij in qs {
            c += count[ij];
        }
        self.query_count.push(c);
    }
}

const EPS: f64 = 1e-6;

/// 各クエリ処理を行うためのデータ
struct Sim {
    n: usize,
    n2: usize,
    m: usize,
    total: usize,
    eps: f64,
    query: Vec<(Vec<usize>, usize)>,
    /// (i, j, resp)
    mined: Vec<(usize, u8)>,
    /// probs[k][S][r] := (kマスに対してクエリして真の値がSであるときに(lb+r)が返ってくる確率, そのln)
    /// ある程度離れると確率はほぼ0になるので、高速化のため、必要な範囲だけ保持している
    probs_lb: Vec<Vec<usize>>,
    probs: Vec<Vec<Vec<(f64, f64)>>>,
    /// ln_probs_query[q][S] := q番目のクエリに対して真の値がSであるときに実際の返答が返ってくる確率のln
    ln_probs_query: Vec<Vec<f64>>,
    /// 失敗した推測履歴
    failed: Vec<Vec<usize>>,
    /// 残クエリ数
    rem: usize,
}

impl Sim {
    fn new(input: &Input) -> Self {
        let mut probs_lb = mat![0; input.n * input.n + 1; input.total + 1];
        let mut probs_ub = 0;
        let mut probs = mat![vec![]; input.n * input.n + 1; input.total + 1];
        for k in 1..=input.n * input.n {
            for S in 0..=input.total {
                let mu = (k as f64 - S as f64) * input.eps + S as f64 * (1.0 - input.eps);
                let sigma = (k as f64 * input.eps * (1.0 - input.eps)).sqrt();
                for r in (0..=mu.round() as usize).rev() {
                    let prob = probability_in_range(mu, sigma, if r == 0 { -100.0 } else { r as f64 - 0.5 }, r as f64 + 0.5);
                    if prob < EPS {
                        probs_lb[k][S] = r + 1;
                        break;
                    }
                    probs[k][S].push((prob, prob.ln()));
                }
                probs[k][S].reverse();
                for r in mu.round() as usize + 1.. {
                    let prob = probability_in_range(mu, sigma, if r == 0 { -100.0 } else { r as f64 - 0.5 }, r as f64 + 0.5);
                    if prob < EPS {
                        break;
                    }
                    probs[k][S].push((prob, prob.ln()));
                    probs_ub.setmax(r + 1);
                }
            }
        }
        eprintln!("n2 = {}", input.n2);
        eprintln!("total = {}", input.total);
        eprintln!("ub = {}", probs_ub);
        Self {
            n: input.n,
            n2: input.n2,
            m: input.m,
            total: input.total,
            eps: input.eps,
            query: vec![],
            mined: vec![],
            probs_lb,
            probs,
            ln_probs_query: vec![],
            failed: vec![],
            rem: input.n * input.n * 2,
        }
    }
    fn ans(&mut self, T: &Vec<usize>) -> bool {
        if self.rem == 0 {
            eprintln!("!log giveup 1 {} {}", self.n, self.m);
            std::process::exit(0);
        }
        self.rem -= 1;
        println!(
            "a {} {}",
            T.len(),
            T.iter().map(|&ij| format!("{} {}", ij / self.n, ij % self.n)).join(" ")
        );
        input_interactive!(ret: usize);
        if ret == 1 {
            return true;
        }
        self.failed.push(T.clone());
        false
    }
    fn query(&mut self, ps: &Vec<usize>) -> usize {
        if self.rem == 0 {
            eprintln!("!log giveup 1 {} {}", self.n, self.m);
            std::process::exit(0);
        }
        self.rem -= 1;
        println!(
            "q {} {}",
            ps.len(),
            ps.iter().map(|&ij| format!("{} {}", ij / self.n, ij % self.n)).join(" ")
        );
        input_interactive!(ret: usize);
        self.query.push((ps.clone(), ret));
        let mut probs = (0..=self.total)
            .map(|S| {
                let k = ps.len();
                let mu = (k as f64 - S as f64) * self.eps + S as f64 * (1.0 - self.eps);
                let sigma = (k as f64 * self.eps * (1.0 - self.eps)).sqrt();
                probability_in_range(mu, sigma, if ret == 0 { -100.0 } else { ret as f64 - 0.5 }, ret as f64 + 0.5).ln()
            })
            .collect_vec();
        for i in 0..probs.len() - 1 {
            if !probs[i].is_infinite() && probs[i + 1].is_infinite() {
                probs[i + 1] = probs[i] - 10.0;
            }
        }
        for i in (1..probs.len()).rev() {
            if !probs[i].is_infinite() && probs[i - 1].is_infinite() {
                probs[i - 1] = probs[i] - 10.0;
            }
        }
        self.ln_probs_query.push(probs);
        ret
    }
    fn mine(&mut self, i: usize, j: usize) -> usize {
        if self.rem == 0 {
            eprintln!("!log giveup 1 {} {}", self.n, self.m);
            std::process::exit(0);
        }
        self.rem -= 1;
        println!("q 1 {} {}", i, j);
        input_interactive!(ret: u8);
        self.mined.push((i * self.n + j, ret));
        ret as usize
    }
    /// 埋蔵量がvsで各ポリオミノの位置がpsのときの対数尤度を計算
    fn ln_prob(&self, state: &State, vs: &Vec<u8>, ps: &Vec<usize>) -> f64 {
        'lp: for qs in &self.failed {
            for &ij in qs {
                if vs[ij] == 0 {
                    continue 'lp;
                }
            }
            let mut size = 0;
            for ij in 0..self.n2 {
                if vs[ij] > 0 {
                    size += 1;
                }
            }
            if size == qs.len() {
                return -1e20;
            }
        }
        let mut prob = 0.0;
        for q in 0..self.query.len() {
            let count = ps
                .iter()
                .enumerate()
                .map(|(p, &ij)| state.pij_to_query_count[p][ij][q])
                .sum::<u8>() as usize;
            prob += self.ln_probs_query[q][count];
        }
        prob
    }
    /// 状態stateのときの対数尤度を計算
    fn ln_prob_state(&self, state: &State) -> f64 {
        'lp: for qs in &self.failed {
            for &ij in qs {
                if state.count[ij] == 0 {
                    continue 'lp;
                }
            }
            let mut size = 0;
            for ij in 0..self.n2 {
                if state.count[ij] > 0 {
                    size += 1;
                }
            }
            if size == qs.len() {
                return -1e20;
            }
        }
        self.ln_probs_query
            .iter()
            .zip(&state.query_count)
            .map(|(ln_probs, &count)| ln_probs[count as usize])
            .sum::<f64>()
    }
    /// 時間切れしたときはBFSで掘る。滅多に実行されない上に相対スコアへの影響が小さいので手抜き
    fn giveup(&mut self) {
        eprintln!("!log giveup 1 {} {}", self.n, self.m);
        let mut que = VecDeque::new();
        que.push_back((self.n / 2, self.n / 2));
        let mut list = vec![];
        let mut rem = self.total;
        let mut used = mat![false; self.n; self.n];
        while let Some((i, j)) = que.pop_front() {
            if !used[i][j].setmax(true) {
                continue;
            }
            let ret = self.mine(i, j);
            if ret > 0 {
                list.push(i * self.n + j);
                rem -= ret;
                if rem == 0 {
                    break;
                }
            }
            for (di, dj) in DIJ {
                let i2 = i + di;
                let j2 = j + dj;
                if i2 < self.n && j2 < self.n {
                    if ret == 0 {
                        que.push_back((i2, j2));
                    } else {
                        que.push_front((i2, j2));
                    }
                }
            }
        }
        self.ans(&list);
    }
}

struct Input {
    n: usize,
    n2: usize,
    m: usize,
    eps: f64,
    ts: Vec<Vec<usize>>,
    masks: Vec<BitSet>,
    max_i: Vec<usize>,
    max_j: Vec<usize>,
    total: usize,
    ps: Vec<usize>,
}

fn read_input() -> Input {
    input_interactive! {
        n: usize,
        m: usize,
        eps: f64,
        ts2: [[(usize, usize)]; m]
    }
    let total = ts2.iter().map(|t| t.len()).sum();
    // 同じ図形かの判定を行うためにポリオミノをソートしておく
    let mut order = (0..m).collect_vec();
    order.sort_by_key(|&p| &ts2[p]);
    let ts2 = order.iter().map(|&p| ts2[p].clone()).collect_vec();
    let max_i = ts2
        .iter()
        .map(|t| t.iter().map(|&(i, _)| i).max().unwrap())
        .collect::<Vec<_>>();
    let max_j = ts2
        .iter()
        .map(|t| t.iter().map(|&(_, j)| j).max().unwrap())
        .collect::<Vec<_>>();
    let ts = ts2
        .iter()
        .map(|t| t.into_iter().map(|(i, j)| i * n + j).collect_vec())
        .collect_vec();
    let masks = ts
        .iter()
        .map(|ts| {
            let mut mask = BitSet::new(n * n);
            for &ij in ts {
                mask.set(ij, true);
            }
            mask
        })
        .collect_vec();
    let mut ps = vec![];
    #[cfg(feature = "local")]
    {
        // testerを改造して真の解も受け取れるようにし、考察に活用
        input_interactive! {
            qs: [(usize, usize); m]
        }
        ps.extend(qs.into_iter().map(|(i, j)| i * n + j));
        ps = order.iter().map(|&p| ps[p]).collect_vec();
    }
    Input {
        n,
        n2: n * n,
        m,
        eps,
        ts,
        masks,
        max_i,
        max_j,
        total,
        ps,
    }
}

impl Input {
    fn get_count(&self, ps: &Vec<usize>) -> Vec<u8> {
        let mut count = vec![0; self.n2];
        for p in 0..ps.len() {
            let pij = ps[p];
            for &ij in &self.ts[p] {
                count[ij + pij] += 1;
            }
        }
        count
    }
    fn get_positives(&self, ps: &Vec<usize>) -> BitSet {
        let mut mask = BitSet::new(self.n2);
        for p in 0..self.m {
            mask |= &(&self.masks[p] << ps[p]);
        }
        mask
    }
}

// ここからライブラリ

pub trait SetMinMax {
    fn setmin(&mut self, v: Self) -> bool;
    fn setmax(&mut self, v: Self) -> bool;
}
impl<T> SetMinMax for T
where
    T: PartialOrd,
{
    fn setmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn setmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

#[macro_export]
macro_rules! mat {
	($($e:expr),*) => { vec![$($e),*] };
	($($e:expr,)*) => { vec![$($e),*] };
	($e:expr; $d:expr) => { vec![$e; $d] };
	($e:expr; $d:expr $(; $ds:expr)+) => { vec![mat![$e $(; $ds)*]; $d] };
}

pub fn get_time() -> f64 {
    static mut STIME: f64 = -1.0;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
    let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
    unsafe {
        if STIME < 0.0 {
            STIME = ms;
        }
        // ローカル環境とジャッジ環境の実行速度差はget_timeで吸収しておくと便利
        #[cfg(feature = "local")]
        {
            (ms - STIME) * 1.0
        }
        #[cfg(not(feature = "local"))]
        {
            ms - STIME
        }
    }
}

fn normal_cdf(x: f64, mean: f64, std_dev: f64) -> f64 {
    0.5 * (1.0 + libm::erf((x - mean) / (std_dev * 1.41421356237)))
}

fn probability_in_range(mean: f64, std_dev: f64, a: f64, b: f64) -> f64 {
    if mean < a {
        return probability_in_range(mean, std_dev, 2.0 * mean - b, 2.0 * mean - a);
    }
    let p_a = normal_cdf(a, mean, std_dev);
    let p_b = normal_cdf(b, mean, std_dev);
    p_b - p_a
}

