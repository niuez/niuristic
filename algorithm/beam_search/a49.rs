// https://atcoder.jp/contests/tessoku-book/submissions/51019518

use rand::prelude::*;
use ettbeam::*;
use proconio::{*, marker::*};

fn main() {
    let input = Input::from_stdin();
    let init_state = State::new(&input);
    let width = 25000;
    let mut beam = BeamSearch::new(500000, TURN + 1, init_state);
    let mut selector = MaxSegSelector::new(width);
    for t in 0..TURN {
        selector.clear();
        eprintln!("turn: {} {:?}", t, beam.state.x);
        beam.extend(&mut selector);
        if t + 1 < TURN {
            eprintln!("cnt {}", selector.selected.len());
            for leaf in selector.selected.iter() {
                beam.select(leaf);
            }
        }
        else {
            let mut i = 0;
            for j in 0..selector.selected.len() {
                if selector.selected[j].score < selector.selected[i].score {
                    i = j;
                }
            }
            let ans = beam.acts(&selector.selected[i]);
            let mut x = [0i16; N];
            eprintln!("score {:?}", selector.selected[i].score);
            eprintln!("ans len {}", ans.len());
            eprintln!("x: {:?}", beam.state.x);
            let mut score = 0;
            for (k, a) in ans.into_iter().enumerate() {
                let v = if a { 1 } else { -1 };
                for &i in input.queries[k].iter() {
                    x[i] += v;
                }
                score += x.iter().filter(|v| **v == 0).count();
                if a {
                    println!("A");
                }
                else {
                    println!("B");
                }
            }
            eprintln!("score = {}", score);
        }
    }
}

const N: usize = 20;
const TURN: usize = 100;

struct State<'a> {
    input: &'a Input,
    t: usize,
    x: [i16; N],
    zeros: i16,
    // table[i][j][k] :=
    // iターン目でj番目の要素の絶対値がkであるときに、そこからj番目の要素を考えて最適に動かしたときの今後得られるスコアの最大値
    table: [[[i16; TURN]; N]; TURN + 1],
    zob: [u64; N],
    now_score: i16,
    now_hash: u64,
}

impl<'a> BeamState for State<'a> {
    type Acts = Vec<bool>;
    type Action = bool;
    // min is better
    type Score = Reverse<i16>;
    fn hash(&self) -> u64 {
        self.now_hash
    }
    fn score(&self) -> Self::Score {
        Reverse(self.now_score)
    }
    fn forward(&mut self, act: &Self::Action) {
        //eprintln!("{} f {}", self.t, act);
        let add = if *act { 1 } else { -1 };
        for i in self.input.queries[self.t] {
            self.now_score -= self.table[self.t][i][self.x[i].abs() as usize];
            self.zeros -= (self.x[i] == 0) as i16;
            self.x[i] += add;
            self.now_hash = self.now_hash.wrapping_add(self.zob[i].wrapping_mul(add as u64));
            self.now_score += self.table[self.t + 1][i][self.x[i].abs() as usize];
            self.zeros += (self.x[i] == 0) as i16;
        }
        self.now_score += self.zeros;
        self.t += 1;
    }
    fn backward(&mut self, act: &Self::Action) {
        //eprintln!("{} b {}", self.t, act);
        let add = if *act { -1 } else { 1 };
        self.t -= 1;
        self.now_score -= self.zeros;
        for i in self.input.queries[self.t] {
            self.now_score -= self.table[self.t + 1][i][self.x[i].abs() as usize];
            self.zeros -= (self.x[i] == 0) as i16;
            self.x[i] += add;
            self.now_hash = self.now_hash.wrapping_add(self.zob[i].wrapping_mul(add as u64));
            self.now_score += self.table[self.t][i][self.x[i].abs() as usize];
            self.zeros += (self.x[i] == 0) as i16;
        }
    }
    fn next_acts(&self) -> Self::Acts {
        //eprintln!("{} {:?} next?", self.t, self.x);
        if self.t == 0 {
            vec![true]
        }
        else {
            vec![false, true]
        }
    }
}

impl<'a> State<'a> {
    fn calc_score(&self) -> i16 {
        let mut s = 0;
        for i in 0..N {
            s += self.table[self.t][i][self.x[i].abs() as usize];
            s += (self.x[i] == 0) as i16;
        }
        s
    }
    fn calc_hash(&self) -> u64 {
        let mut h = 0;
        for i in 0..N {
            h += self.zob[i] * self.x[i] as u64;
        }
        h
    }
    fn new(input: &'a Input) -> Self {
        let Input { ref queries, .. } = input;
        let mut table = [[[0; TURN]; N]; TURN + 1];
        for i in (0..TURN).rev() {
            for &j in &queries[i] {
                table[i][j][0] = table[i + 1][j][1];
                for k in 1..TURN {
                    table[i][j][k] = table[i + 1][j][k - 1] + (k == 1) as i16;
                }
            }
            for j in 0..N {
                if queries[i].contains(&j) {
                    continue;
                }
                for k in 0..TURN {
                    table[i][j][k] = table[i + 1][j][k] + (k == 0) as i16;
                }
            }
        }


        let mut rng = rand_pcg::Pcg64Mcg::new(768);
        let mut zob = [!0; N];
        for i in 0..N {
            zob[i] = rng.gen();
        }
        let mut s = Self {
            input,
            t: 0,
            x: [0; N],
            zeros: N as i16,
            table,
            zob,
            now_score: 0,
            now_hash: 0,
        };
        s.now_score = s.calc_score();
        s.now_hash = s.calc_hash();
        s
    }
}

struct Input {
    queries: [[usize; 3]; TURN],
}

impl Input {
    fn from_stdin() -> Self {
        input! {
            turn: usize,
            q: [[Usize1; 3]; turn],
        }
        let mut nq = [[!0; 3]; TURN];
        for i in 0..TURN {
            for j in 0..3 {
                nq[i][j] = q[i][j];
            }
        }
        Self {
            queries: nq,
        }
    }
}

use std::cmp::Reverse;
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

#[allow(unused)]
pub mod ettbeam {
    use super::*;
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


