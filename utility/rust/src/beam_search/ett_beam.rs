#[allow(unused)]
pub mod ettbeam {
    use crate::util::nop_hash::{ NopHashSet, NopHashMap };
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
