#![allow(non_snake_case, dead_code)]

fn main() {
    get_time();
    let input = Input::stdin(true);

    let rects = input.local.actual.clone();
    let mut area_acc = vec![0; input.N + 1];
    for i in (0..input.N).rev() {
        area_acc[i] = area_acc[i + 1] + rects[i].h * rects[i].w;
    }
    let init_state = State {
        turn: 0,
        max_x: 573000,
        area_acc: area_acc.clone(),
        max_y: 0,
        rects,
        col_idx: vec![],
        col_placed: vec![],
        dead_spaces: 0,
        free_spaces: 0,
        dead_col: 0,
        dead_line: 0,
        dead_flag: 0,
        sh: 0,
    };
    let width = 10000;
    let mut beam = BeamSearch::new(600000, input.N + 1, init_state);
    let mut selector = SortSelector::new(width);
    for t in 0..input.N {
        selector.clear();
        eprintln!("turn: {}", t);
        beam.extend(&mut selector);
        if t + 1 < input.N {
            selector.select(&mut beam);
        }
        else {
            let mut i = 0;
            for j in 0..selector.selected.len() {
                if selector.selected[j].score < selector.selected[i].score {
                    i = j;
                }
            }
            let ans = beam.acts(&selector.selected[i]);
            println!("{}", input.N);
            let mut last = vec![-1; input.N];
            for t in 0..input.N {
                let ci = ans[t].col;
                let rot = ans[t].rot;
                println!("# {} {} {} {:?}", ans[t].dead_space, ans[t].free_space, area_acc[t + 1], ans[t].before_next_idx);
                println!("{} {} {} {}", t, rot as usize, 'U', last[ci]);
                last[ci] = t as isize;
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
struct ColumnPlaced {
    x_left: i64,
    x_right: i64,
    y: i64,
}

#[derive(Clone)]
struct State {
    turn: usize,
    max_x: i64,
    area_acc: Vec<i64>,
    max_y: i64,
    rects: Vec<Rect>,
    col_idx: Vec<usize>,
    col_placed: Vec<Vec<ColumnPlaced>>,
    dead_spaces: i64,
    free_spaces: i64,
    dead_col: usize,
    dead_line: i64,
    dead_flag: u64,
    sh: u64,
}

#[derive(Default, Clone, Debug)]
struct Action {
    col: usize,
    rot: bool,
    before_next_idx: Vec<(usize, usize)>,
    dead_space: i64,
    free_space: i64,
    next_max_y: i64,
    before_max_y: i64,
    dead_col_delta: usize,
    dead_line_delta: i64,
    before_next_dead_flag: (u64, u64),
    placed: ColumnPlaced,
}

use ettbeam::*;

impl BeamState for State {
    type Action = Action;
    type Score = i64;
    type Acts = Vec<Action>;
    fn forward(&mut self, act: &Self::Action) {
        if act.col == self.col_placed.len() {
            self.col_placed.push(vec![]);
            self.col_idx.push(0);
        }
        self.col_placed[act.col].push(act.placed.clone());
        for i in 0..act.before_next_idx.len() {
            self.col_idx[act.col - 1 - i] = act.before_next_idx[i].1;
        }
        self.dead_spaces += act.dead_space;
        self.free_spaces += act.free_space;
        self.max_y = act.next_max_y;
        self.dead_col += act.dead_col_delta;
        self.dead_line += act.dead_line_delta;
        self.dead_flag = act.before_next_dead_flag.1;
        self.turn += 1;
        self.sh += 1;
    }
    fn backward(&mut self, act: &Self::Action) {
        self.turn -= 1;
        self.dead_spaces -= act.dead_space;
        self.free_spaces -= act.free_space;
        self.max_y = act.before_max_y;
        self.dead_col -= act.dead_col_delta;
        self.dead_line -= act.dead_line_delta;
        self.dead_flag = act.before_next_dead_flag.0;
        for i in 0..act.before_next_idx.len() {
            self.col_idx[act.col - 1 - i] = act.before_next_idx[i].0;
        }
        self.col_placed[act.col].pop();
        if self.col_placed[act.col].len() == 0 {
            self.col_placed.pop();
            self.col_idx.pop();
        }
    }
    fn next_acts(&self) -> Self::Acts {
        let mut acts = vec![];
        for c in self.dead_col..=self.col_placed.len() {
            if (1 << c) & self.dead_flag > 0 { continue; }
            for r in [false, true] {
                let (h, w) = self.rects[self.turn].get(r);
                let x = self.col_placed.get(c).map(|placed| placed.last().unwrap().x_right).unwrap_or(0);
                if x + h > self.max_x { continue; }
                if c == 0 {
                    acts.push(Action {
                        col: c,
                        rot: r,
                        before_next_idx: vec![],
                        dead_space: 0,
                        free_space: (self.max_y.max(w) - self.max_y) * self.max_x - h * w,
                        before_max_y: self.max_y,
                        next_max_y: self.max_y.max(w),
                        dead_col_delta: 0,
                        dead_line_delta: 0,
                        before_next_dead_flag: (self.dead_flag, self.dead_flag),
                        placed: ColumnPlaced { x_left: x, x_right: x + h, y: w },
                    });
                }
                else {
                    let mut ref_c = c;
                    let mut dead_space = 0;
                    let mut dead_x = 0;
                    let mut undead_x = 0;
                    let mut max_y = 0;
                    let mut free_space = 0;
                    let mut before_next_idx = vec![];
                    let mut next_dead_flag = self.dead_flag;
                    let mut last_x_right = 0;
                    while ref_c > 0 {
                        ref_c -= 1;
                        let mut idx = self.col_idx[ref_c];
                        while idx < self.col_placed[ref_c].len() {
                            let l = x.max(self.col_placed[ref_c][idx].x_left);
                            let r = (x + h).min(self.col_placed[ref_c][idx].x_right);
                            if max_y < self.col_placed[ref_c][idx].y {
                                dead_x += undead_x;
                                undead_x = r - l;
                                dead_space += dead_x * (self.col_placed[ref_c][idx].y - max_y);
                                max_y = self.col_placed[ref_c][idx].y;
                            }
                            else {
                                dead_x += r - l;
                                dead_space += (r - l) * (max_y - self.col_placed[ref_c][idx].y);
                            }
                            last_x_right = self.col_placed[ref_c][idx].x_right;
                            if x + h < self.col_placed[ref_c][idx].x_right {
                                break;
                            }
                            idx += 1;
                        }
                        before_next_idx.push((self.col_idx[ref_c], idx));
                        if idx < self.col_placed[ref_c].len() {
                            break;
                        }
                        next_dead_flag |= 1 << ref_c;
                    }
                    if last_x_right < x + h {
                        dead_space += (x + h - last_x_right) * max_y;
                    }
                    free_space += (self.max_y.max(max_y + w) - self.max_y) * self.max_x - h * w;
                    if self.max_x - (x + h) >= 30000 {
                        acts.push(Action {
                            col: c,
                            rot: r,
                            before_next_idx,
                            before_max_y: self.max_y,
                            next_max_y: self.max_y.max(max_y + w),
                            dead_space,
                            free_space,
                            dead_col_delta: 0,
                            dead_line_delta: 0,
                            before_next_dead_flag: (self.dead_flag, next_dead_flag),
                            placed: ColumnPlaced { x_left: x, x_right: x + h, y: max_y + w },
                        });
                    }
                    else {
                        let mut line = self.dead_line;
                        {
                            dead_space += (max_y + w - line) * (self.max_x - (x + h));
                            line = max_y + w;
                        }
                        acts.push(Action {
                            col: c,
                            rot: r,
                            before_next_idx,
                            before_max_y: self.max_y,
                            next_max_y: self.max_y.max(max_y + w),
                            dead_space,
                            free_space,
                            dead_col_delta: c - self.dead_col + 1,
                            dead_line_delta: line - self.dead_line,
                            before_next_dead_flag: (self.dead_flag, next_dead_flag),
                            placed: ColumnPlaced { x_left: x, x_right: x + h, y: max_y + w },
                        });
                    }
                }
            }
        }

        acts
    }
    fn score(&self) -> Self::Score {
        self.dead_spaces + (self.free_spaces - self.dead_spaces - self.area_acc[self.turn]).max(0)
    }
    fn hash(&self) -> u64 {
        self.sh
    }
}

#[derive(Clone)]
struct Rect {
    w: i64,
    h: i64,
}

impl Rect {
    fn get(&self, rot: bool) -> (i64, i64) {
        if rot { (self.w, self.h) } else { (self.h, self.w) }
    }
}

#[derive(Clone)]
struct Local {
    actual: Vec<Rect>,
    diff: Vec<Rect>,
}

impl Local {
    fn empty() -> Local {
        Local {
            actual: Vec::new(),
            diff: Vec::new(),
        }
    }
    fn stdin(N: usize, T: usize) -> Local {
        input_interactive! {
            act: [(i64, i64); N],
            diff: [(i64, i64); T],
        }
        Local {
            actual: act.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
            diff: diff.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
        }
    }
}

#[derive(Clone)]
struct Input {
    N: usize,
    T: usize,
    sigma: i64,
    rects: Vec<Rect>,
    local: Local,
}


impl Input {
    fn stdin(is_local: bool) -> Input {
        input_interactive! {
            N: usize,
            T: usize,
            sigma: i64,
            rect: [(i64, i64); N],
        }
        Input {
            N,
            T,
            sigma,
            rects: rect.into_iter().map(|a| Rect { h: a.0, w: a.1 }).collect_vec(),
            local: if is_local { Local::stdin(N, T) } else { Local::empty() },
        }
    }
}

use itertools::Itertools;
use proconio::{input, input_interactive};
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



