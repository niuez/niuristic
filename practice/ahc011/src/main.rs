#![allow(non_snake_case, dead_code)]

const PPCNT: Lazy<Vec<i64>> = Lazy::new(|| {
    // i -(d)-> j
    let mut p = vec![0; 16 * 16 * 4];
    for i in 0..16usize {
        for j in 0..16 {
            // R
            p[(i * 16 + j) * 4 + 0] = ((i & (1 << 2)) > 0 && (j & (1 << 0)) > 0) as i64;
            // L
            p[(i * 16 + j) * 4 + 1] = ((i & (1 << 0)) > 0 && (j & (1 << 2)) > 0) as i64;
            // D
            p[(i * 16 + j) * 4 + 2] = ((i & (1 << 3)) > 0 && (j & (1 << 1)) > 0) as i64;
            // U
            p[(i * 16 + j) * 4 + 3] = ((i & (1 << 1)) > 0 && (j & (1 << 3)) > 0) as i64;
        }
    }
    p
});

fn pop_count(i: u8, j: u8, d: usize) -> i64 {
    - PPCNT[(i as usize * 16 + j as usize) * 4 + d]
}

fn main() {
    get_time();
    let input = Input::stdin();
    let init_state = State::new(&input);
    let width = 2000;
    let mut beam = BeamSearch::new(500000, input.T + 1, init_state);
    let mut selector = MaxSegSelector::new(width);
    for t in 0..input.T {
        selector.clear();
        beam.extend(&mut selector);
        if t + 1 < input.T {
            let mut min = std::i64::MAX;
            for leaf in selector.selected.iter() {
                beam.select(leaf);
                min = min.min(leaf.score);
            }
            eprintln!("cnt {} {}", selector.selected.len(), min);
        }
        else {
            let mut i = 0;
            for j in 0..selector.selected.len() {
                if selector.selected[j].score < selector.selected[i].score {
                    i = j;
                }
            }
            let ans = beam.acts(&selector.selected[i]);
            eprintln!("score {:?}", selector.selected[i].score);
            let CC = vec!['R', 'L', 'D', 'U'];
            for ai in ans.into_iter() {
                print!("{}", CC[ai as usize]);
            }
            println!();
        }
    }
}

use ettbeam::*;


struct State {
    S: usize,
    C: Vec<u8>,
    sc: i64,
    sh: u64,
    zob: Vec<u64>,
    acts: Vec<Vec<u8>>,
    dirs: Vec<(usize, char)>,
}

impl State {
    fn new(input: &Input) -> Self {
        let dirs = vec![
            (1, 'R'),
            (!0, 'L'),
            (input.N, 'D'),
            ((!0usize).wrapping_mul(input.N), 'U'),
        ];
        let mut mt = Mt::new(768);
        let zob = (0..input.N*input.N*16).map(|_| mt.gen::<u64>()).collect_vec();
        let mut sc = 0;
        let mut sh = 0;
        let mut acts = vec![vec![]; input.N * input.N];
        let C = input.C.clone();
        for i in 0..input.N {
            for j in 0..input.N {
                let v = i * input.N + j;
                sh ^= zob[C[v] as usize + v * 16];
                if i + 1 < input.N {
                    acts[v].push(2);
                }
                if i > 0 {
                    acts[v].push(3);
                    sc += pop_count(C[v], C[v - input.N], 3);
                }
                if j + 1 < input.N {
                    acts[v].push(0);
                }
                if j > 0 {
                    acts[v].push(1);
                    sc += pop_count(C[v], C[v - 1], 1);
                }
            }
        }
        State {
            S: input.S,
            C,
            sc,
            sh,
            zob,
            acts,
            dirs,
        }
    }
}

impl BeamState for State {
    type Action = u8;
    type Score = i64;
    type Acts = Vec<u8>;
    fn forward(&mut self, act: &Self::Action) {
        let v = self.S;
        let u = v.wrapping_add(self.dirs[*act as usize].0);
        for &ai in &self.acts[u] {
            self.sc -= pop_count(self.C[u], self.C[u.wrapping_add(self.dirs[ai as usize].0)], ai as usize);
        }
        self.sh ^= self.zob[u * 16 + self.C[u] as usize];
        self.C.swap(u, v);
        for &ai in &self.acts[v] {
            self.sc += pop_count(self.C[v], self.C[v.wrapping_add(self.dirs[ai as usize].0)], ai as usize);
        }
        self.sh ^= self.zob[v * 16 + self.C[v] as usize];
        self.S = u;
    }
    fn backward(&mut self, act: &Self::Action) {
        self.forward(&(*act ^ 1));
    }
    fn next_acts(&self) -> Self::Acts {
        self.acts[self.S].clone()
    }
    fn score(&self) -> Self::Score {
        self.sc
    }
    fn hash(&self) -> u64 {
        self.sh
    }
}

struct Input {
    N: usize,
    T: usize,
    S: usize,
    C: Vec<u8>,
}

impl Input {
    fn stdin() -> Self {
        use proconio::marker::Chars;
        input! {
            N: usize,
            T: usize,
            t: [Chars; N],
        }

        let mut C = vec![0u8; N*N];
        let mut S = !0;
        for i in 0..N {
            for j in 0..N {
                if t[i][j] == '0' {
                    S = i * N + j;
                }
                C[i * N + j] = t[i][j].to_digit(16).unwrap() as u8;
            }
        }
        Input {
            N,
            T,
            S,
            C
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
        type Score: Copy + Ord + Default;
        type Acts: IntoIterator<Item=Self::Action>;
        fn forward(&mut self, act: &Self::Action);
        fn backward(&mut self, act: &Self::Action);
        fn next_acts(&self) -> Self::Acts;
        fn score(&self) -> Self::Score;
        fn hash(&self) -> u64;
    }


    pub struct Leaf<State: BeamState> {
        pub t: usize,
        pub score: State::Score,
        pub hash: u64,
    }

    pub trait Selector<State: BeamState> {
        fn push(&mut self, leaf: Leaf<State>);
    }
    
    #[derive(Default, Clone, Debug)]
    pub struct Edge<Action: Default + Clone + std::fmt::Debug> {
        act: Action,
        down: bool,
        selected: bool,
    }

    pub struct BeamSearch<State: BeamState> {
        pub state: State,
        cur: Vec<Edge<State::Action>>,
        sz: usize,
        nxt: Vec<Edge<State::Action>>,
        downs: Vec<usize>,
    }

    impl<State: BeamState> BeamSearch<State> {
        pub fn new(max_width: usize, max_depth: usize, root_state: State) -> Self {
            let mut x = Self {
                state: root_state,
                cur: vec![Edge::default(); max_width],
                sz: 1,
                nxt: vec![Edge::default(); max_width],
                downs: vec![!0; max_depth],
            };
            x.cur[0].selected = true;
            x
        }

        pub fn select(&mut self, leaf: &Leaf<State>) {
            self.cur[leaf.t].selected = true;
        }

        pub fn extend<S: Selector<State>>(&mut self, selector: &mut S) {
            let Self { ref mut state, ref cur, sz, ref mut nxt, ref mut downs } = *self;
            let mut ni = 0;
            let mut dl = 0;
            let mut dr = 0;
            for i in 0..sz {
                let e = &cur[i];
                if e.selected {
                    while dl < dr {
                        nxt[ni] = Edge { act: cur[downs[dl]].act.clone(), down: true, selected: false, };
                        ni += 1;
                        state.forward(&cur[downs[dl]].act);
                        dl += 1;
                    }
                    for next_act in state.next_acts() {
                        state.forward(&next_act);
                        nxt[ni] = Edge { act: next_act.clone(), down: true, selected: false };
                        ni += 1;

                        let leaf = Leaf { t: ni, score: state.score(), hash: state.hash() };
                        selector.push(leaf);

                        state.backward(&next_act);
                        nxt[ni].down = false;
                        nxt[ni].selected = false;
                        ni += 1;
                    }
                }
                // 余分にdownがあるので打ち切る
                if i + 1 == sz {
                    for d in (0..dl).rev() {
                        state.backward(&cur[downs[d]].act);
                        nxt[ni].down = false;
                        nxt[ni].selected = false;
                        ni += 1;
                    }
                    break;
                }
                if e.down {
                    downs[dr] = i;
                    dr += 1;
                }
                else {
                    // up
                    if dl == dr {
                        dl -= 1;
                        state.backward(&cur[downs[dl]].act);
                        nxt[ni].down = false;
                        nxt[ni].selected = false;
                        ni += 1;
                    }
                    dr -= 1;
                }
            }
            std::mem::swap(&mut self.cur, &mut self.nxt);
            self.sz = ni;
        }

        pub fn acts(&mut self, leaf: &Leaf<State>) -> Vec<State::Action> {
            let mut acts = vec![];
            eprintln!("{} {} {}", self.cur.len(), self.sz, leaf.t);
            for i in 0..leaf.t {
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

    pub struct SortSelector<State: BeamState> {
        width: usize,
        pub selected: Vec<Leaf<State>>,
        map: NopHashSet<u64>,
    }

    impl<State: BeamState> Selector<State> for SortSelector<State> {
        fn push(&mut self, leaf: Leaf<State>) {
            self.selected.push(leaf);
        }
    }

    impl<State: BeamState> SortSelector<State> {
        pub fn new(width: usize) -> Self {
            Self {
                width,
                selected: Vec::new(),
                map: NopHashSet::default(),
            }
        }
        pub fn clear(&mut self) {
            self.selected.clear();
            self.map.clear();
        }
        pub fn select(&mut self, beam: &mut BeamSearch<State>) {
            if self.selected.len() > self.width {
                self.selected.select_nth_unstable_by_key(self.width, |e| e.score);
                self.selected.truncate(self.width);
            }
            self.selected.sort_unstable_by_key(|a| a.score);
            for leaf in self.selected.iter() {
                if self.map.insert(leaf.hash) {
                    beam.select(leaf);
                }
            }
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
        pub selected: Vec<Leaf<State>>,
        hash_to_idx: NopHashMap<u64, usize>,
        seg: MaxSeg<State::Score>,
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
        pub fn clear(&mut self) {
            self.selected.clear();
            self.hash_to_idx.clear();
            self.full = false;
        }
    }

    impl<State: BeamState> Selector<State> for MaxSegSelector<State> {
        fn push(&mut self, leaf: Leaf<State>) {
            if self.full && leaf.score >= self.seg.all().0 {
                return;
            }
            if let Some(bi) = self.hash_to_idx.get(&leaf.hash) {
                // hashが同じものが存在したとき
                if leaf.score < self.selected[*bi].score {
                    if self.full {
                        self.seg.update(*bi, leaf.score);
                    }
                    self.selected[*bi] = leaf;
                }
            }
            else if self.full {
                let i = self.seg.all().1;
                self.hash_to_idx.remove(&self.selected[i].hash);
                self.hash_to_idx.insert(leaf.hash, i);
                self.seg.update(i, leaf.score);
                self.selected[i] = leaf;
            }
            else {
                let i = self.selected.len();
                self.hash_to_idx.insert(leaf.hash, i);
                self.selected.push(leaf);

                if self.width == self.selected.len() {
                    self.full = true;
                    for i in 0..self.width {
                        self.seg.unfix_set(i, self.selected[i].score);
                    }
                    self.seg.fix();
                }
            }
        }
    }
}


