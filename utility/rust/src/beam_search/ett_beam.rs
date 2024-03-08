#[allow(unused)]
pub mod ettbeam {
    use crate::util::nop_hash::NopHashMap;
    use std::cmp::Ordering;

    pub trait BeamState {
        type Action: Default + Clone;
        type Score: Copy + Ord + Default;
        type Acts: IntoIterator<Item=Self::Action>;
        fn score_cmp(a: &Self::Score, b: &Self::Score) -> Ordering;
        fn forward(&mut self, act: &Self::Action);
        fn backward(&mut self, act: &Self::Action);
        fn next_acts(&self) -> Self::Acts;
        fn score(&self) -> Self::Score;
        fn hash(&self) -> u64;
    }


    pub struct Leaf<State: BeamState> {
        t: usize,
        score: State::Score,
        hash: u64,
    }

    pub trait Selector<State: BeamState> {
        fn push(&mut self, leaf: Leaf<State>);
    }
    
    #[derive(Default, Clone)]
    pub struct Edge<Action: Default + Clone> {
        act: Action,
        down: bool,
        selected: bool,
    }

    pub struct BeamSearch<State: BeamState> {
        state: State,
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
            x.select(0);
            x
        }

        pub fn select(&mut self, t: usize) {
            self.cur[t].selected = true;
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

                        let leaf = Leaf { t: i, score: state.score(), hash: state.hash() };
                        selector.push(leaf);

                        state.backward(&next_act);
                        nxt[ni].down = false;
                        nxt[ni].selected = false;
                        ni += 1;
                    }
                }
                // 余分にdownがあるので打ち切る
                if i + 1 == sz {
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
                    }
                    dr -= 1;
                }
            }
        }
    }

    struct MaxSeg<S: Copy + Ord + Default> {
        n: usize,
        seg: Vec<(S, usize)>,
    }

    // min is better
    impl<S: Copy + Ord + Default> MaxSeg<S> {
        fn new(width: usize) -> Self {
            let mut n = 1;
            while n < width { n *= 2; }
            Self {
                n,
                seg: vec![(S::default(), !0); n * 2],
            }
        }
        fn at(&self, i: usize) -> (S, usize) {
            self.seg[i + self.n]
        }
        fn unfix_set(&mut self, i: usize, score: S) {
            self.seg[i + self.n] = (score, i);
        }
        fn fix(&mut self) {
            for i in (1..self.n).rev() {
                self.seg[i] = self.seg[i << 1].max(self.seg[(i << 1) + 1]);
            }
        }

        fn update(&mut self, mut i: usize, score: S) {
            self.seg[i + self.n] = (score, i);
            i += self.n;
            while i > 1 {
                i >>= 1;
                self.seg[i] = self.seg[i << 1].max(self.seg[(i << 1) + 1]);
            }
        }
        fn all(&self) -> (S, usize) {
            self.seg[1]
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
