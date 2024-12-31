#![allow(non_snake_case, dead_code)]
use ettbeam::*;
fn main() {
    get_time();
    let input = Input::stdin();
    let state = State::new(&input);
    let width = 750;
    let mut beam = BeamSearch::new(width, 50000, M + 1, state);
    for t in 0..M {
        eprintln!("turn {}", t);
        beam.extend();
        if t + 1 < M {
        }
        else {
            let mut i = 0;
            for j in 0..beam.selector.selected.len() {
                if beam.selector.selected[j].0.score() < beam.selector.selected[i].0.score() {
                    i = j;
                }
            }
            let mut ans = beam.acts(beam.selector.selected[i].1);
            ans.push(beam.selector.selected.swap_remove(i).0.into_action());

            let mut last_w = !0;
            let mut v = input.start;
            let state = &beam.state;
            for t in 0..ans.len() {
                let w = ans[t].word as usize;
                let l = if last_w == !0 { 0 } else { state.sames[last_w][w] };
                let mut ki = ans[t].K_from as usize;
                let kj = ans[t].K_to as usize;
                for i in l..state.dp[w].len() {
                    let next_v = state.K[state.T[w][i] as usize][ki];
                    println!("{} {}", next_v / N, next_v % N);
                    ki = state.dp[w][i][kj][ki].1;
                    v = next_v;
                }
                assert_eq!(v, ans[t].pos as usize);
                last_w = w;
            }
        }
    }
}

#[derive(Clone)]
struct Typing {
    from: usize,
    to: usize,
    cost: i32,
    trail: Vec<usize>,
}

struct State {
    move_cost: Vec<Vec<i32>>,
    dp: Vec<Vec<Vec<Vec<(i32, usize)>>>>,
    move_dp: Vec<Vec<Vec<(i32, usize)>>>,
    base_cost: Vec<i32>,
    sames: Vec<Vec<usize>>,
    T: Vec<Vec<u8>>,
    K: Vec<Vec<usize>>,
    iK: Vec<usize>,

    used_zob: Vec<u64>,
    pos_zob: Vec<u64>,

    word: usize,
    pos: usize,
    cost: i16,
    used: Vec<bool>,
    zob: u64,

    hist: Vec<(u8, u8)>,
}

#[derive(Clone, Default, Debug)]
struct Change<T: Clone> {
    before: T,
    after: T,
}

#[derive(Clone, Debug, Default)]
struct Action {
    word: u8,
    pos: u8,
    K_from: u8,
    K_to: u8,
    cost: i16,
}

#[derive(Clone, Debug, Default)]
struct Candidate {
    act: Action,
    cost: i16,
    zob: u64,
}

impl BeamCandidate for Candidate {
    type Score = i16;
    type Action = Action;
    fn hash(&self) -> u64 {
        self.zob
    }
    fn score(&self) -> Self::Score {
        self.cost
    }
    fn into_action(self) -> Self::Action {
        self.act
    }
}

impl BeamState for State {
    type Action = Action;
    type Candidate = Candidate;
    type Candidates = Vec<Candidate>;
    fn forward(&mut self, act: &Action) {
        self.hist.push((self.word as u8, self.pos as u8));
        self.zob ^= self.used_zob[act.word as usize] ^ self.pos_zob[self.pos as usize] ^ self.pos_zob[act.pos as usize];
        self.word = act.word as usize;
        self.pos = act.pos as usize;
        self.cost += act.cost;
        self.used[act.word as usize] = true;
    }
    fn backward(&mut self, act: &Action) {
        let (w, p) = self.hist.pop().unwrap();
        self.zob ^= self.used_zob[act.word as usize] ^ self.pos_zob[p as usize] ^ self.pos_zob[act.pos as usize];
        self.word = w as usize;
        self.pos = p as usize;
        self.cost -= act.cost;
        self.used[act.word as usize] = false;
    }
    fn next_candidates(&self) -> Self::Candidates {
        let mut cands = vec![];
        for w in 0..M {
            if !self.used[w] {
                let l = if self.word >= M { 0 } else { self.sames[self.word][w] };
                for i in 0..self.dp[w][l].len() {
                    let mut min_j = (std::i32::MAX, !0);
                    if l == 0 {
                        min_j = self.move_dp[w][i][self.pos];
                    }
                    else {
                        min_j = self.dp[w][l - 1][i][self.iK[self.pos]];
                    }
                    let act = Action {
                        word: w as u8,
                        pos: self.K[*self.T[w].last().unwrap() as usize][i] as u8,
                        K_from: min_j.1 as u8,
                        K_to: i as u8,
                        cost: (min_j.0 - self.base_cost[w]) as i16,
                    };
                    let cand = Candidate {
                        cost: act.cost + self.cost,
                        zob: self.zob ^ self.used_zob[act.word as usize] ^ self.pos_zob[self.pos as usize] ^ self.pos_zob[act.pos as usize],
                        act,
                    };
                    cands.push(cand);
                }
            }
        }
        cands
    }
}

impl State {
    fn new(input: &Input) -> State {
        let mut move_cost = vec![vec![0; N * N]; N * N];
        for i in 0..N * N {
            for j in 0..N * N {
                move_cost[i][j] = ((i / N).abs_diff(j / N) + (i % N).abs_diff(j % N) + 1) as i32;
            }
        }
        let mut all_dp = Vec::new();
        let mut all_move_dp = Vec::new();
        let mut base_cost = vec![std::i32::MAX; M];
        for wi in 0..M {
            let L = input.T[wi].len();
            let sz = input.K[input.T[wi][L - 1] as usize].len();
            let mut dp = vec![vec![vec![]]; L];
            dp[L - 1] = vec![vec![(std::i32::MAX >> 1, !0); sz]; sz];
            for i in 0..sz {
                dp[L - 1][i][i] = (0, !0);
            }
            for s in (0..L - 1).rev() {
                let before_k = &input.K[input.T[wi][s + 1] as usize];
                let next_k = &input.K[input.T[wi][s] as usize];
                dp[s] = vec![vec![(std::i32::MAX >> 1, !0); next_k.len()]; sz];
                for si in 0..sz {
                    for i in 0..before_k.len() {
                        for j in 0..next_k.len() {
                            dp[s][si][j] = dp[s][si][j].min(
                                (dp[s + 1][si][i].0 + move_cost[before_k[i]][next_k[j]], i)
                            );
                        }
                    }
                }
            }

            let mut move_dp = vec![vec![(std::i32::MAX, 0usize); N * N]; sz];
            let last_k = &input.K[input.T[wi][0] as usize];
            for i in 0..sz {
                for j in 0..last_k.len() {
                    for k in 0..N*N {
                        move_dp[i][k] = move_dp[i][k].min((dp[0][i][j].0 + move_cost[last_k[j]][k], j));
                    }
                }
            }

            for j in 0..sz {
                for i in 0..input.K[input.T[wi][0] as usize].len() {
                    base_cost[wi] = base_cost[wi].min(dp[0][j][i].0);
                }
            }
            all_dp.push(dp);
            all_move_dp.push(move_dp);

            /*

            for s in 0..input.K[input.T[wi][0] as usize].len() {
                for t in 0..input.K[input.T[wi][L - 1] as usize].len() {
                    let mut v = s;
                    let mut trail = vec![input.K[input.T[wi][0] as usize][v]];
                    for i in 1..L {
                        v = dp[i - 1][t][v].1;
                        trail.push(input.K[input.T[wi][i] as usize][v]);
                    }
                    assert!(v == t);

                    typings[wi].push(Typing {
                        from: trail[0],
                        to: trail[L - 1],
                        cost: dp[0][t][s].0,
                        trail,
                    });
                }
            }
            */
        }

        let mut sames = vec![vec![0; M]; M];
        for i in 0..M {
            for j in 0..M {
                let mut suf = 0u64;
                let mut pre = 0u64;
                let mut f = 1u64;
                for l in 1..=5 {
                    suf = suf + f * input.T[i][5 - l] as u64;
                    pre = pre * 26 + input.T[j][l - 1] as u64;
                    f *= 26;
                    if suf == pre {
                        sames[i][j] = l;
                    }
                }
            }
        }

        let mut mt = Mt::new(768);
        let used_zob = (0..M).map(|_| mt.gen::<u64>()).collect_vec();
        let pos_zob = (0..N*N).map(|_| mt.gen::<u64>()).collect_vec(); 

        State {
            move_cost,
            dp: all_dp,
            move_dp: all_move_dp,
            base_cost,
            sames,
            K: input.K.clone(),
            iK: input.iK.clone(),
            T: input.T.clone(),

            used_zob,
            pos_zob,

            word: (!0u8) as usize,
            pos: input.start,
            cost: 0,
            used: vec![false; M],
            zob: 0,

            hist: vec![],
        }
    }
}

const N: usize = 15;
const M: usize = 200;

struct Input {
    start: usize,
    A: Vec<u8>,
    T: Vec<Vec<u8>>,
    K: Vec<Vec<usize>>,
    iK: Vec<usize>,
}

impl Input {
    fn stdin() -> Self {
        input! {
            n_: usize,
            m_: usize,
            si: usize,
            sj: usize,
            a: [Chars; N],
            t: [Chars; M],
        }
        let mut A = vec![0u8; N * N];
        let mut K = vec![vec![]; 26];
        let mut iK = vec![0usize; N * N];
        for i in 0..N {
            for j in 0..N {
                A[i * N + j] = a[i][j] as u8 - 'A' as u8;
                iK[i * N + j] = K[A[i * N + j] as usize].len();
                K[A[i * N + j] as usize].push(i * N + j);
            }
        }
        let T = t.into_iter()
            .map(|t| t.into_iter().map(|c| c as u8 - 'A' as u8).collect_vec())
            .collect_vec();
        Self {
            start: si * N + sj,
            A,
            T,
            K,
            iK,
        }
    }
}

use itertools::Itertools;
use proconio::{input, input_interactive, marker::Chars};
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

#[allow(unused)]
pub mod ettbeam {
    use std::collections::{HashMap,HashSet};
    use core::hash::BuildHasherDefault;
    use core::hash::Hasher;

#[derive(Default)]
    pub struct NopHasher{
        hash:u64,
    }
    impl Hasher for NopHasher{
        fn write(&mut self,_:&[u8]){
            panic!();
        }

        #[inline]
        fn write_u64(&mut self,n:u64){
            self.hash=n;
        }

        #[inline]
        fn finish(&self)->u64{
            self.hash
        }
    }

    pub type NopHashMap<K,V>=HashMap<K,V,BuildHasherDefault<NopHasher>>;
    pub type NopHashSet<V>=HashSet<V,BuildHasherDefault<NopHasher>>;
    use std::cmp::Ordering;

    pub trait BeamState {
        type Action: Default + Clone + std::fmt::Debug;
        type Candidate: BeamCandidate<Action=Self::Action>;
        type Candidates: IntoIterator<Item=Self::Candidate>;
        fn forward(&mut self, act: &Self::Action);
        fn backward(&mut self, act: &Self::Action);
        fn next_candidates(&self) -> Self::Candidates;
    }

    pub trait BeamCandidate {
        type Action;
        type Score: Default + Copy + Ord;
        fn into_action(self) -> Self::Action;
        fn score(&self) -> Self::Score;
        fn hash(&self) -> u64;
    }


    #[derive(Default, Clone, Debug)]
    pub struct Edge<Action: Default + Clone + std::fmt::Debug> {
        act: Action,
        down: bool,
    }

    pub struct BeamSearch<State: BeamState> {
        pub state: State,
        pub cur: Vec<Edge<State::Action>>,
        pub sz: usize,
        nxt: Vec<Edge<State::Action>>,
        downs: Vec<usize>,
        pub selector: MaxSegSelector<State>,
    }

    impl<State: BeamState> BeamSearch<State> {
        pub fn new(width: usize, max_width: usize, max_depth: usize, root_state: State) -> Self {
            let x = Self {
                state: root_state,
                cur: vec![Edge::default(); max_width],
                sz: 0,
                nxt: vec![Edge::default(); max_width],
                downs: vec![!0; max_depth],
                selector: MaxSegSelector::new(width),
            };
            x
        }

        pub fn extend(&mut self) {
            let Self { ref mut state, ref cur, sz, ref mut nxt, ref mut downs, ref mut selector,} = *self;
            if selector.selected.is_empty() {
                // first time
                assert_eq!(sz, 0);
                for next_cand in state.next_candidates() {
                    selector.push(next_cand, 0);
                }
            }
            else {
                let mut ni = 0;
                let mut dl = 0;
                let mut dr = 0;
                let mut selected = {
                    let mut v = selector.clear();
                    v.sort_unstable_by(|x, y| y.1.cmp(&x.1));
                    v
                };
                // eprintln!("selected {:?}", selected.iter().map(|x| x.1).collect_vec());
                for i in 0..=sz {
                    while selected.len() > 0 && selected.last().unwrap().1 == i {
                        let cand = selected.pop().unwrap().0;
                        while dl < dr {
                            nxt[ni] = Edge { act: cur[downs[dl]].act.clone(), down: true, };
                            ni += 1;
                            state.forward(&cur[downs[dl]].act);
                            dl += 1;
                        }
                        nxt[ni] = Edge { act: cand.into_action(), down: true, };
                        state.forward(&nxt[ni].act);
                        ni += 1;

                        for next_cand in state.next_candidates() {
                            selector.push(next_cand, ni);
                        }
                        state.backward(&nxt[ni - 1].act);

                        nxt[ni].down = false;
                        ni += 1;
                    }
                    if i == sz {
                        for d in (0..dl).rev() {
                            state.backward(&cur[downs[d]].act);
                            nxt[ni].down = false;
                            ni += 1;
                        }
                        break;
                    }
                    if cur[i].down {
                        downs[dr] = i;
                        dr += 1;
                    }
                    else {
                        if dl == dr {
                            dl -= 1;
                            state.backward(&cur[downs[dl]].act);
                            nxt[ni].down = false;
                            ni += 1;
                        }
                        dr -= 1;
                    }
                }
                std::mem::swap(&mut self.cur, &mut self.nxt);
                self.sz = ni;
            }
        }

        pub fn acts(&mut self, leaf: usize) -> Vec<State::Action> {
            let mut acts = vec![];
            eprintln!("{} {} {}", self.cur.len(), self.sz, leaf);
            for i in 0..leaf {
                let e = &self.cur[i];
                if e.down {
                    self.state.forward(&e.act);
                    acts.push(e.act.clone())
                }
                else {
                    let act = acts.pop().unwrap();
                    self.state.backward(&act);
                }
            }
            acts
        }
    }

    struct MaxSeg<S: Copy + Ord + Default> {
        n: usize,
        seg: Vec<(bool, S, usize)>,
    }

    // min is better
    // max is want
    impl<S: Copy + Ord + Default> MaxSeg<S> {
        fn new(width: usize) -> Self {
            let mut n = 1;
            while n < width { n *= 2; }
            Self {
                n,
                seg: vec![(false, S::default(), !0); n * 2],
            }
        }
        fn at(&self, i: usize) -> (S, usize) {
            (self.seg[i + self.n].1, self.seg[i + self.n].2)
        }
        fn unfix_set(&mut self, i: usize, score: S) {
            self.seg[i + self.n] = (true, score, i);
        }
        fn fix(&mut self) {
            for i in (1..self.n).rev() {
                self.seg[i] = self.seg[i << 1].max(self.seg[(i << 1) + 1]);
            }
        }

        fn update(&mut self, mut i: usize, score: S) {
            self.seg[i + self.n] = (true, score, i);
            i += self.n;
            while i > 1 {
                i >>= 1;
                self.seg[i] = self.seg[i << 1].max(self.seg[(i << 1) + 1]);
            }
        }
        fn all(&self) -> (S, usize) {
            (self.seg[1].1, self.seg[1].2)
        }
    }
    
    pub struct MaxSegSelector<State: BeamState> {
        width: usize,
        full: bool,
        pub selected: Vec<(State::Candidate, usize)>,
        hash_to_idx: NopHashMap<u64, usize>,
        seg: MaxSeg<<State::Candidate as BeamCandidate>::Score>,
    }

    impl<State: BeamState> MaxSegSelector<State> {
        pub fn new(width: usize) -> Self {
            Self {
                width,
                full: false,
                selected: vec![],
                hash_to_idx: NopHashMap::default(),
                seg: MaxSeg::new(width),
            }
        }
        pub fn clear(&mut self) -> Vec<(State::Candidate, usize)> {
            self.hash_to_idx.clear();
            self.full = false;
            std::mem::replace(&mut self.selected, vec![])
        }

        fn push(&mut self, cand: State::Candidate, parent: usize) {
            let score = cand.score();
            let hash = cand.hash();
            if self.full && score >= self.seg.all().0 {
                return;
            }
            if let Some(bi) = self.hash_to_idx.get(&hash) {
                // hashが同じものが存在したとき
                if score < self.selected[*bi].0.score() {
                    if self.full {
                        self.seg.update(*bi, score);
                    }
                    self.selected[*bi] = (cand, parent);
                }
            }
            else if self.full {
                let i = self.seg.all().1;
                self.hash_to_idx.remove(&self.selected[i].0.hash());
                self.hash_to_idx.insert(hash, i);
                self.seg.update(i, score);
                self.selected[i] = (cand, parent);
            }
            else {
                let i = self.selected.len();
                self.hash_to_idx.insert(hash, i);
                self.selected.push((cand, parent));

                if self.width == self.selected.len() {
                    self.full = true;
                    for i in 0..self.width {
                        self.seg.unfix_set(i, self.selected[i].0.score());
                    }
                    self.seg.fix();
                }
            }
        }
    }
}



