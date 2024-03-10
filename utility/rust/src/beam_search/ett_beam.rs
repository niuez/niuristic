#[allow(unused)]
pub mod ettbeam {
    use crate::util::nop_hash::{ NopHashSet, NopHashMap };
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



