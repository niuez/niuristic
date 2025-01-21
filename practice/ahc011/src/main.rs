#![allow(non_snake_case, dead_code)]

use std::process::exit;

use ettbeam::*;

const PPCNT: Lazy<Vec<i64>> = Lazy::new(|| {
    // i -(d)-> j
    let mut p = vec![0; 16 * 16 * 4];
    for i in 0..16usize {
        for j in 0..16 {
            p[(i * 16 + j) * 4 + 0] = ((i & (1 << 0)) > 0 && (j & (1 << 2)) > 0) as i64;
            p[(i * 16 + j) * 4 + 1] = ((i & (1 << 1)) > 0 && (j & (1 << 3)) > 0) as i64;
            p[(i * 16 + j) * 4 + 2] = ((i & (1 << 2)) > 0 && (j & (1 << 0)) > 0) as i64;
            p[(i * 16 + j) * 4 + 3] = ((i & (1 << 3)) > 0 && (j & (1 << 1)) > 0) as i64;
        }
    }
    p
});

fn pop_count(i: u8, j: u8, d: usize) -> i64 {
    PPCNT[(i as usize * 16 + j as usize) * 4 + d]
}

fn main() {
    get_time();
    let input = Input::stdin();
    let mut state = comp_sa::State::new(&input);
    let place = state.solve(1.5, &input);

    let ch = " ╴╵┘╶─└┴╷┐│┤┌┬├┼".chars().collect_vec();

    for i in 0..input.N {
        for j in 0..input.N {
            eprint!("{:>2} ", place[i * input.N + j]);
        }
        eprintln!();
    }

    for i in 0..input.N {
        for j in 0..input.N {
            eprint!("{}", ch[input.C[place[i * input.N + j]] as usize]);
        }
        eprintln!();
    }

    let MAX = 2 * input.N * input.N * input.N;
    let MAX = 0;
    let mut beam = BeamSearch::new(10000, 1000000, MAX + 1, AnsState::new(&input, place, state.check));
    for turn in 0..MAX {
        beam.extend();
        let mut min = std::i64::MAX;
        for i in 0..beam.selector.selected.len() {
            min = min.min(beam.selector.selected[i].0.score);
            if beam.selector.selected[i].0.score == 0 || MAX == turn + 1 {
                let mut ans = beam.acts(beam.selector.selected[i].1);
                ans.push(beam.selector.selected[i].0.action.clone());
                let ch = ['L', 'U', 'R', 'D'];
                for a in ans {
                    print!("{}", ch[a.dir as usize]);
                }
                println!();
                exit(0);
            }
        }
        eprintln!("turn {} {}", turn, min);
    }
}

struct AnsState {
    place: Vec<usize>,
    S: usize,
    delta: Vec<Vec<(usize, usize)>>,
    dist: Vec<Vec<i64>>, // dist[self.place[i]][i]
    dist_sum: i64,

    moving: [usize; 4],

    place_zob: Vec<Vec<u64>>,
    zob: u64,
}

impl AnsState {
    fn new(input: &Input, goal: Vec<usize>, delta: Vec<Vec<(usize, usize)>>) -> Self {
        let N = input.N;
        let mut dist = vec![vec![0; N*N]; N*N];
        for i in 0..N {
            for j in 0..N {
                for a in 0..N {
                    for b in 0..N {
                        dist[goal[i * N + j]][a * N + b] = (i.abs_diff(a) + j.abs_diff(b)) as i64;
                    }
                }
            }
        }
        let mut dist_sum = 0;
        for i in 0..N*N {
            if input.S != i {
                dist_sum += dist[i][i];
            }
        }

        let mut place_zob = vec![vec![0u64; N*N]; N*N];
        let mut mt = Mt::new(768);
        for i in 0..N*N {
            for j in 0..N*N {
                place_zob[i][j] = mt.gen::<u64>();
            }
        }

        let moving = [1usize.wrapping_neg(), N.wrapping_neg(), 1usize, N];
        AnsState {
            place: (0..N*N).collect_vec(),
            S: input.S,
            delta,
            dist,
            dist_sum,
            moving,
            place_zob,
            zob: 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AnsAction {
    dir: u8,
}

impl BeamState for AnsState {
    type Score = i64;
    type Action = AnsAction;
    type Candidates = Vec<BeamCandidate<AnsAction, i64>>;
    fn forward(&mut self, act: &Self::Action) {
        let mut next = self.S.wrapping_add(self.moving[act.dir as usize]);
        self.zob ^= self.place_zob[self.place[next]][next];
        self.dist_sum -= self.dist[self.place[next]][next];
        self.place.swap(self.S, next);
        std::mem::swap(&mut self.S, &mut next);
        self.zob ^= self.place_zob[self.place[next]][next];
        self.dist_sum += self.dist[self.place[next]][next];
    }
    fn backward(&mut self, act: &Self::Action) {
        let mut next = self.S.wrapping_add(self.moving[(act.dir ^ 2) as usize]);
        self.zob ^= self.place_zob[self.place[next]][next];
        self.dist_sum -= self.dist[self.place[next]][next];
        self.place.swap(self.S, next);
        std::mem::swap(&mut self.S, &mut next);
        self.zob ^= self.place_zob[self.place[next]][next];
        self.dist_sum += self.dist[self.place[next]][next];
    }
    fn next_candidates(&self) -> Self::Candidates {
        let mut cands = vec![];
        for &(next, d) in &self.delta[self.S] {
            let cand = BeamCandidate {
                action: AnsAction { dir: d as u8 },
                score: self.dist_sum
                    - self.dist[self.place[next]][next]
                    + self.dist[self.place[next]][self.S],
                hash: self.zob
                    ^ self.place_zob[self.place[next]][next]
                    ^ self.place_zob[self.place[next]][self.S],
            };
            cands.push(cand);

        }
        cands
    }
}

struct State {
    C: Vec<u8>,
}


mod comp_sa {

use super::*;

pub struct State {
    place: Vec<usize>,
    C: Vec<u8>,
    S: usize,
    delta: Vec<Vec<(usize, usize)>>,
    pub check: Vec<Vec<(usize, usize)>>,
    outer: Vec<Vec<usize>>,
    dist: Vec<Vec<i64>>, // dist[self.place[i]][i]
}

impl State {
    pub fn new(input: &Input) -> State {
        let N = input.N;
        let mut delta = vec![vec![]; N * N];
        let mut check = vec![vec![]; N * N];
        let mut outer = vec![vec![]; N * N];
        for i in 0..N {
            for j in 0..N {
                let v = i * N + j;
                if j > 0 {
                    check[v].push((v - 1, 0));
                }
                else {
                    outer[v].push(0);
                }
                if j + 1 < N {
                    delta[v].push((v + 1, 2));
                    check[v].push((v + 1, 2));
                }
                else {
                    outer[v].push(2);
                }
                if i > 0 {
                    check[v].push((v - N, 1));
                }
                else {
                    outer[v].push(1);
                }
                if i + 1 < N {
                    delta[v].push((v + N, 3));
                    check[v].push((v + N, 3));
                }
                else {
                    outer[v].push(3);
                }
            }
        }

        let mut dist = vec![vec![0; N*N]; N*N];
        for i in 0..N {
            for j in 0..N {
                for a in 0..N {
                    for b in 0..N {
                        dist[i * N + j][a * N + b] = (i.abs_diff(a) + j.abs_diff(b)) as i64;
                    }
                }
            }
        }

        State {
            S: input.S,
            place: (0..N*N).collect_vec(),
            C: input.C.clone(),
            delta,
            check,
            outer,
            dist,
        }
    }

    fn count_components(&self) -> usize {
        static mut uf: UnionFind = UnionFind { par: [1usize.wrapping_neg(); 128] };
        unsafe { uf.clear(); }
        let mut cnt = self.place.len();
        for i in 0..self.place.len() {
            for &(j, d) in &self.delta[i] {
                if pop_count(self.C[i], self.C[j], d) > 0 {
                    if unsafe { uf.unite(i, j) } {
                        cnt -= 1;
                    }
                }
            }
        }
        cnt
    }

    fn check_ok_placement(&self, i: usize) -> bool {
        for &d in &self.outer[i] {
            if self.C[i] & (1 << d) > 0 {
                return false;
            }
        }
        for &(j, d) in &self.check[i] {
            if pop_count(self.C[i], self.C[j], d) > 0 {
                return true;
            }
        }
        false
    }

    pub fn solve(&mut self, TL: f64, input: &Input) -> Vec<usize> {
        let mut now_comps = self.count_components();
        let mut now_dist = 0;
        let mut swap_cnt = 0;

        let mut mt = Mt::new(768);
        let START = get_time();
        let T0 = 2.0f64;
        let T1 = 0.001f64;
        let mut T = T0;
        let mut iter = 0;

        let mut best = (std::i64::MAX, self.place.clone());

        loop {
            let t = get_time();
            if t >= TL {
                break;
            }
            let p = (get_time() - START) / (TL - START);
            iter += 1;
            if iter % 1000 == 0 {
                eprintln!("{} {} {}", iter, now_comps, now_dist);
                T = T0.powf(1.0 - p) * T1.powf(p);
            }
            {
                let i = mt.gen_range(0..self.place.len());
                let j = mt.gen_range(0..self.place.len());
                if i == j {
                    continue;
                }
                let dist_delta =
                      - self.dist[self.place[i]][j] + self.dist[self.place[i]][i]
                      - self.dist[self.place[j]][i] + self.dist[self.place[j]][j];
                self.place.swap(i, j);
                self.C.swap(i, j);
                if !self.check_ok_placement(i) || !self.check_ok_placement(j) {
                    self.place.swap(i, j);
                    self.C.swap(i, j);
                    continue;
                }

                let next_comps = self.count_components();
                let delta = (now_comps as f64 - next_comps as f64) + dist_delta as f64 * 0.00001;
                if delta >= 0.0 || mt.gen_bool((delta / T).exp()) {
                    if self.C[i] == 0 { self.S = i; }
                    if self.C[j] == 0 { self.S = j; }
                    now_comps = next_comps;
                    now_dist -= dist_delta;
                    swap_cnt += 1;

                    if now_comps == 2 && swap_cnt % 2 == (self.dist[input.S][self.S] % 2) as usize && best.0 > now_dist {
                        best.0 = now_dist;
                        best.1 = self.place.clone();
                    }
                }
                else {
                    self.place.swap(i, j);
                    self.C.swap(i, j);
                }
            }
        }

        if best.0 == std::i64::MAX {
            best.1 = self.place.clone();
        }

        best.1
    }
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

pub struct UnionFind {
    par: [usize; 128],
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            par: [1usize.wrapping_neg(); 128],
        }
    }

    pub fn clear(&mut self) {
        self.par.fill(1usize.wrapping_neg());
    }

    pub fn root(&mut self, v: usize) -> usize {
        if self.par[v] >= 128 {
            v
        }
        else {
            let r = self.root(self.par[v]);
            self.par[v] = r;
            self.par[v]
        }
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let a = self.root(a);
        let b = self.root(b);
        if a == b { false }
        else if self.par[a] > self.par[b] {
            self.par[b] = self.par[b].wrapping_add(self.par[a]);
            self.par[a] = b;
            true
        }
        else {
            self.par[a] = self.par[a].wrapping_add(self.par[b]);
            self.par[b] = a;
            true
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
        type Score: Default + Copy + Ord;
        type Candidates: IntoIterator<Item=BeamCandidate<Self::Action, Self::Score>>;
        fn forward(&mut self, act: &Self::Action);
        fn backward(&mut self, act: &Self::Action);
        fn next_candidates(&self) -> Self::Candidates;
    }

    pub struct BeamCandidate<A, S: Default + Copy + Ord> {
        pub action: A,
        pub score: S,
        pub hash: u64,
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
                        nxt[ni] = Edge { act: cand.action, down: true, };
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
        pub selected: Vec<(BeamCandidate<State::Action, State::Score>, usize)>,
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
        pub fn clear(&mut self) -> Vec<(BeamCandidate<State::Action, State::Score>, usize)> {
            self.hash_to_idx.clear();
            self.full = false;
            std::mem::replace(&mut self.selected, vec![])
        }

        fn push(&mut self, cand: BeamCandidate<State::Action, State::Score>, parent: usize) {
            let score = cand.score;
            let hash = cand.hash;
            if self.full && score >= self.seg.all().0 {
                return;
            }
            if let Some(bi) = self.hash_to_idx.get(&hash) {
                // hashが同じものが存在したとき
                if score < self.selected[*bi].0.score {
                    if self.full {
                        self.seg.update(*bi, score);
                    }
                    self.selected[*bi] = (cand, parent);
                }
            }
            else if self.full {
                let i = self.seg.all().1;
                self.hash_to_idx.remove(&self.selected[i].0.hash);
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
                        self.seg.unfix_set(i, self.selected[i].0.score);
                    }
                    self.seg.fix();
                }
            }
        }
    }
}
